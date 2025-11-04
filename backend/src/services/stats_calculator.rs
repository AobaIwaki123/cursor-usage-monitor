use crate::models::{
    usage_data::UsageData,
    stats::{PeakUsageStats, CostEfficiencyStats, UsageTrendStats, UsagePercentiles, ComprehensiveStats}
};
use chrono::{DateTime, Timelike};
use std::collections::HashMap;

pub struct StatsCalculator;

impl StatsCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Calculate peak usage statistics (hours, days)
    /// Requirements: 7.1
    pub fn calculate_peak_usage(&self, data: &[UsageData]) -> PeakUsageStats {
        if data.is_empty() {
            return PeakUsageStats {
                peak_hour: 0,
                peak_day: String::new(),
                peak_tokens_per_hour: 0,
                peak_cost_per_day: 0.0,
            };
        }

        let mut hourly_tokens: HashMap<u8, u32> = HashMap::new();
        let mut daily_costs: HashMap<String, f64> = HashMap::new();

        for usage in data {
            // Parse date and extract hour and day
            if let Ok(datetime) = DateTime::parse_from_rfc3339(&usage.date) {
                let hour = datetime.hour() as u8;
                let day = datetime.format("%Y-%m-%d").to_string();

                // Accumulate tokens by hour
                *hourly_tokens.entry(hour).or_insert(0) += usage.total_tokens;
                
                // Accumulate costs by day
                *daily_costs.entry(day).or_insert(0.0) += usage.cost;
            }
        }

        // Find peak hour
        let (peak_hour, peak_tokens_per_hour) = hourly_tokens
            .iter()
            .max_by_key(|(_, &tokens)| tokens)
            .map(|(&hour, &tokens)| (hour, tokens))
            .unwrap_or((0, 0));

        // Find peak day
        let (peak_day, peak_cost_per_day) = daily_costs
            .iter()
            .max_by(|(_, &cost_a), (_, &cost_b)| cost_a.partial_cmp(&cost_b).unwrap())
            .map(|(day, &cost)| (day.clone(), cost))
            .unwrap_or((String::new(), 0.0));

        PeakUsageStats {
            peak_hour,
            peak_day,
            peak_tokens_per_hour,
            peak_cost_per_day,
        }
    }

    /// Calculate cost efficiency metrics
    /// Requirements: 7.2
    pub fn calculate_cost_efficiency(&self, data: &[UsageData]) -> CostEfficiencyStats {
        if data.is_empty() {
            return CostEfficiencyStats {
                cost_per_token: 0.0,
                cost_per_request: 0.0,
                cache_savings: 0.0,
            };
        }

        let total_cost: f64 = data.iter().map(|d| d.cost).sum();
        let total_tokens: u32 = data.iter().map(|d| d.total_tokens).sum();
        let total_requests = data.len() as f64;

        let cost_per_token = if total_tokens > 0 {
            total_cost / total_tokens as f64
        } else {
            0.0
        };

        let cost_per_request = if total_requests > 0.0 {
            total_cost / total_requests
        } else {
            0.0
        };

        // Calculate cache savings based on cache read tokens
        let total_cache_read: u32 = data.iter().map(|d| d.cache_read).sum();
        let total_input_without_cache: u32 = data.iter().map(|d| d.input_without_cache).sum();
        
        let cache_savings = if total_input_without_cache + total_cache_read > 0 {
            (total_cache_read as f64 / (total_input_without_cache + total_cache_read) as f64) * 100.0
        } else {
            0.0
        };

        CostEfficiencyStats {
            cost_per_token,
            cost_per_request,
            cache_savings,
        }
    }

    /// Calculate usage trends and growth rate
    /// Requirements: 7.3, 7.6
    pub fn calculate_usage_trends(&self, data: &[UsageData]) -> UsageTrendStats {
        if data.is_empty() {
            return UsageTrendStats {
                daily_growth_rate: 0.0,
                usage_pattern: "stable".to_string(),
                usage_percentiles: UsagePercentiles {
                    median: 0,
                    p95: 0,
                    p99: 0,
                },
            };
        }

        // Group data by day and calculate daily totals
        let mut daily_usage: HashMap<String, u32> = HashMap::new();
        for usage in data {
            if let Ok(datetime) = DateTime::parse_from_rfc3339(&usage.date) {
                let day = datetime.format("%Y-%m-%d").to_string();
                *daily_usage.entry(day).or_insert(0) += usage.total_tokens;
            }
        }

        // Calculate growth rate
        let mut sorted_days: Vec<_> = daily_usage.iter().collect();
        sorted_days.sort_by_key(|(day, _)| *day);

        let daily_growth_rate = if sorted_days.len() > 1 {
            let first_day_usage = *sorted_days.first().unwrap().1 as f64;
            let last_day_usage = *sorted_days.last().unwrap().1 as f64;
            let days_diff = sorted_days.len() as f64 - 1.0;
            
            if first_day_usage > 0.0 && days_diff > 0.0 {
                ((last_day_usage / first_day_usage).powf(1.0 / days_diff) - 1.0) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Determine usage pattern
        let usage_pattern = if daily_growth_rate > 5.0 {
            "increasing".to_string()
        } else if daily_growth_rate < -5.0 {
            "decreasing".to_string()
        } else {
            "stable".to_string()
        };

        // Calculate percentiles
        let usage_percentiles = self.calculate_percentiles(data);

        UsageTrendStats {
            daily_growth_rate,
            usage_pattern,
            usage_percentiles,
        }
    }

    /// Calculate usage percentiles for distribution statistics
    /// Requirements: 7.6
    fn calculate_percentiles(&self, data: &[UsageData]) -> UsagePercentiles {
        if data.is_empty() {
            return UsagePercentiles {
                median: 0,
                p95: 0,
                p99: 0,
            };
        }

        let mut token_counts: Vec<u32> = data.iter().map(|d| d.total_tokens).collect();
        token_counts.sort_unstable();

        let len = token_counts.len();
        
        let median = if len % 2 == 0 {
            (token_counts[len / 2 - 1] + token_counts[len / 2]) / 2
        } else {
            token_counts[len / 2]
        };

        let p95_index = ((len as f64) * 0.95).ceil() as usize - 1;
        let p95 = token_counts.get(p95_index.min(len - 1)).copied().unwrap_or(0);

        let p99_index = ((len as f64) * 0.99).ceil() as usize - 1;
        let p99 = token_counts.get(p99_index.min(len - 1)).copied().unwrap_or(0);

        UsagePercentiles {
            median,
            p95,
            p99,
        }
    }

    /// Calculate all comprehensive statistics
    /// Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6
    pub fn calculate_comprehensive_stats(&self, data: &[UsageData]) -> ComprehensiveStats {
        ComprehensiveStats {
            peak_usage: self.calculate_peak_usage(data),
            cost_efficiency: self.calculate_cost_efficiency(data),
            usage_trends: self.calculate_usage_trends(data),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::usage_data::UsageData;

    fn create_test_data() -> Vec<UsageData> {
        vec![
            UsageData {
                date: "2024-01-01T10:00:00Z".to_string(),
                kind: "Included".to_string(),
                model: "auto".to_string(),
                max_mode: false,
                input_with_cache: 100,
                input_without_cache: 50,
                cache_read: 25,
                output_tokens: 75,
                total_tokens: 250,
                cost: 0.05,
            },
            UsageData {
                date: "2024-01-01T14:00:00Z".to_string(),
                kind: "Included".to_string(),
                model: "gpt-4".to_string(),
                max_mode: true,
                input_with_cache: 200,
                input_without_cache: 100,
                cache_read: 50,
                output_tokens: 150,
                total_tokens: 500,
                cost: 0.15,
            },
            UsageData {
                date: "2024-01-02T10:00:00Z".to_string(),
                kind: "Included".to_string(),
                model: "auto".to_string(),
                max_mode: false,
                input_with_cache: 150,
                input_without_cache: 75,
                cache_read: 30,
                output_tokens: 95,
                total_tokens: 350,
                cost: 0.08,
            },
        ]
    }

    #[test]
    fn test_calculate_peak_usage() {
        let calculator = StatsCalculator::new();
        let data = create_test_data();
        
        let peak_stats = calculator.calculate_peak_usage(&data);
        
        // Peak hour should be 10 (two entries at 10:00)
        assert_eq!(peak_stats.peak_hour, 10);
        assert_eq!(peak_stats.peak_tokens_per_hour, 600); // 250 + 350
        
        // Peak day should be 2024-01-01 (higher cost)
        assert_eq!(peak_stats.peak_day, "2024-01-01");
        assert_eq!(peak_stats.peak_cost_per_day, 0.20); // 0.05 + 0.15
    }

    #[test]
    fn test_calculate_peak_usage_empty_data() {
        let calculator = StatsCalculator::new();
        let data = vec![];
        
        let peak_stats = calculator.calculate_peak_usage(&data);
        
        assert_eq!(peak_stats.peak_hour, 0);
        assert_eq!(peak_stats.peak_tokens_per_hour, 0);
        assert_eq!(peak_stats.peak_day, "");
        assert_eq!(peak_stats.peak_cost_per_day, 0.0);
    }

    #[test]
    fn test_calculate_cost_efficiency() {
        let calculator = StatsCalculator::new();
        let data = create_test_data();
        
        let cost_efficiency = calculator.calculate_cost_efficiency(&data);
        
        // Total cost: 0.05 + 0.15 + 0.08 = 0.28
        // Total tokens: 250 + 500 + 350 = 1100
        // Total requests: 3
        assert!((cost_efficiency.cost_per_token - 0.28 / 1100.0).abs() < 0.0001);
        assert!((cost_efficiency.cost_per_request - 0.28 / 3.0).abs() < 0.0001);
        
        // Cache savings: total_cache_read / (total_input_without_cache + total_cache_read) * 100
        // (25 + 50 + 30) / (50 + 100 + 75 + 25 + 50 + 30) * 100 = 105 / 330 * 100 ≈ 31.82%
        assert!((cost_efficiency.cache_savings - 31.818181818181817).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_cost_efficiency_empty_data() {
        let calculator = StatsCalculator::new();
        let data = vec![];
        
        let cost_efficiency = calculator.calculate_cost_efficiency(&data);
        
        assert_eq!(cost_efficiency.cost_per_token, 0.0);
        assert_eq!(cost_efficiency.cost_per_request, 0.0);
        assert_eq!(cost_efficiency.cache_savings, 0.0);
    }

    #[test]
    fn test_calculate_usage_trends() {
        let calculator = StatsCalculator::new();
        let data = create_test_data();
        
        let usage_trends = calculator.calculate_usage_trends(&data);
        
        // Growth rate from day 1 (600 tokens) to day 2 (350 tokens) should be negative
        assert!(usage_trends.daily_growth_rate < 0.0);
        assert_eq!(usage_trends.usage_pattern, "decreasing");
        
        // Check percentiles
        let mut sorted_tokens = vec![250, 350, 500];
        sorted_tokens.sort();
        assert_eq!(usage_trends.usage_percentiles.median, 350);
        assert_eq!(usage_trends.usage_percentiles.p95, 500);
        assert_eq!(usage_trends.usage_percentiles.p99, 500);
    }

    #[test]
    fn test_calculate_percentiles() {
        let calculator = StatsCalculator::new();
        let data = create_test_data();
        
        let percentiles = calculator.calculate_percentiles(&data);
        
        assert_eq!(percentiles.median, 350);
        assert_eq!(percentiles.p95, 500);
        assert_eq!(percentiles.p99, 500);
    }

    #[test]
    fn test_calculate_percentiles_empty_data() {
        let calculator = StatsCalculator::new();
        let data = vec![];
        
        let percentiles = calculator.calculate_percentiles(&data);
        
        assert_eq!(percentiles.median, 0);
        assert_eq!(percentiles.p95, 0);
        assert_eq!(percentiles.p99, 0);
    }

    #[test]
    fn test_calculate_comprehensive_stats() {
        let calculator = StatsCalculator::new();
        let data = create_test_data();
        
        let comprehensive_stats = calculator.calculate_comprehensive_stats(&data);
        
        // Verify all components are calculated
        assert_eq!(comprehensive_stats.peak_usage.peak_hour, 10);
        assert!(comprehensive_stats.cost_efficiency.cost_per_token > 0.0);
        assert!(comprehensive_stats.usage_trends.daily_growth_rate < 0.0);
    }
}