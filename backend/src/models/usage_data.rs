use serde::{Deserialize, Serialize};

// Core data structures - will be fully implemented in task 2.3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageData {
    pub date: String,
    pub kind: String,
    pub model: String,
    pub max_mode: bool,
    pub input_with_cache: u32,
    pub input_without_cache: u32,
    pub cache_read: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
    pub cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageSummary {
    pub total_cost: f64,
    pub total_tokens: u32,
    pub average_cost_per_day: f64,
    pub most_used_model: String,
    pub date_range: DateRange,
    pub model_breakdown: Vec<ModelStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStats {
    pub model: String,
    pub total_requests: u32,
    pub total_tokens: u32,
    pub total_cost: f64,
    pub average_tokens_per_request: f64,
    pub cache_hit_rate: f64, // percentage (0-100), calculated as cache_read / total_input * 100
    pub cache_savings: f64, // monetary savings from cache usage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    pub start: String,
    pub end: String,
}