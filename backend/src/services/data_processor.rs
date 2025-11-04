use crate::models::usage_data::{UsageData, UsageSummary, ModelStats, DateRange};
use std::collections::HashMap;

// Data processing service for calculating summaries and merging data
pub struct DataProcessor;

impl DataProcessor {
    pub fn new() -> Self {
        Self
    }

    /// Calculate comprehensive usage summary from usage data
    pub fn calculate_summary(&self, data: &[UsageData]) -> UsageSummary {
        if data.is_empty() {
            return UsageSummary {
                total_cost: 0.0,
                total_tokens: 0,
                average_cost_per_day: 0.0,
                most_used_model: String::new(),
                date_range: DateRange {
                    start: String::new(),
                    end: String::new(),
                },
                model_breakdown: Vec::new(),
            };
        }

        let total_cost = data.iter().map(|d| d.cost).sum::<f64>();
        let total_tokens = data.iter().map(|d| d.total_tokens).sum::<u32>();

        // Calculate date range
        let dates: Vec<&String> = data.iter().map(|d| &d.date).collect();
        let start_date = dates.iter().min().unwrap().to_string();
        let end_date = dates.iter().max().unwrap().to_string();
        
        // Calculate unique days for average cost per day
        let unique_days: std::collections::HashSet<&String> = dates.into_iter().collect();
        let days_count = unique_days.len() as f64;
        let average_cost_per_day = if days_count > 0.0 { total_cost / days_count } else { 0.0 };

        // Calculate model breakdown and find most used model
        let model_breakdown = self.calculate_model_stats(data);
        let most_used_model = model_breakdown
            .iter()
            .max_by_key(|stats| stats.total_requests)
            .map(|stats| stats.model.clone())
            .unwrap_or_default();

        UsageSummary {
            total_cost,
            total_tokens,
            average_cost_per_day,
            most_used_model,
            date_range: DateRange {
                start: start_date,
                end: end_date,
            },
            model_breakdown,
        }
    }

    /// Calculate detailed statistics for each model
    pub fn calculate_model_stats(&self, data: &[UsageData]) -> Vec<ModelStats> {
        let mut model_data: HashMap<String, Vec<&UsageData>> = HashMap::new();
        
        // Group data by model
        for usage in data {
            model_data.entry(usage.model.clone()).or_default().push(usage);
        }

        let mut stats = Vec::new();
        
        for (model, model_usage) in model_data {
            let total_requests = model_usage.len() as u32;
            let total_tokens = model_usage.iter().map(|u| u.total_tokens).sum::<u32>();
            let total_cost = model_usage.iter().map(|u| u.cost).sum::<f64>();
            
            let average_tokens_per_request = if total_requests > 0 {
                total_tokens as f64 / total_requests as f64
            } else {
                0.0
            };

            // Calculate cache efficiency
            let cache_efficiency = self.calculate_cache_efficiency(model_usage);

            stats.push(ModelStats {
                model,
                total_requests,
                total_tokens,
                total_cost,
                average_tokens_per_request,
                cache_efficiency,
            });
        }

        // Sort by total requests (most used first)
        stats.sort_by(|a, b| b.total_requests.cmp(&a.total_requests));
        stats
    }

    /// Calculate cache efficiency as percentage of cache hits
    fn calculate_cache_efficiency(&self, model_usage: Vec<&UsageData>) -> f64 {
        let total_input_tokens: u32 = model_usage.iter()
            .map(|u| u.input_with_cache + u.input_without_cache)
            .sum();
        
        let total_cache_read: u32 = model_usage.iter()
            .map(|u| u.cache_read)
            .sum();

        if total_input_tokens + total_cache_read == 0 {
            return 0.0;
        }

        // Cache efficiency = cache_read / (total_input + cache_read) * 100
        (total_cache_read as f64 / (total_input_tokens + total_cache_read) as f64) * 100.0
    }

    /// Merge existing data with new data, removing duplicates and sorting by date
    pub fn merge_data(&self, existing: Vec<UsageData>, new: Vec<UsageData>) -> Vec<UsageData> {
        let mut combined = existing;
        combined.extend(new);

        // Remove duplicates based on date, model, and cost (assuming these uniquely identify a record)
        combined.sort_by(|a, b| {
            a.date.cmp(&b.date)
                .then_with(|| a.model.cmp(&b.model))
                .then_with(|| a.cost.partial_cmp(&b.cost).unwrap_or(std::cmp::Ordering::Equal))
        });

        combined.dedup_by(|a, b| {
            a.date == b.date && 
            a.model == b.model && 
            a.cost == b.cost &&
            a.total_tokens == b.total_tokens
        });

        combined
    }

    /// Validate usage data for consistency
    pub fn validate_usage_data(&self, data: &[UsageData]) -> Result<(), String> {
        for (index, usage) in data.iter().enumerate() {
            // Validate token calculation
            let calculated_total = usage.input_with_cache + usage.input_without_cache + 
                                 usage.cache_read + usage.output_tokens;
            
            if calculated_total != usage.total_tokens {
                return Err(format!(
                    "Token calculation mismatch at index {}. Sum of individual tokens ({}) doesn't match Total Tokens ({})",
                    index, calculated_total, usage.total_tokens
                ));
            }

            // Validate non-negative values
            if usage.cost < 0.0 {
                return Err(format!("Negative cost at index {}: {}", index, usage.cost));
            }

            // Validate date format (basic check)
            if usage.date.is_empty() {
                return Err(format!("Empty date at index {}", index));
            }
        }

        Ok(())
    }
}