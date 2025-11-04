use serde::{Deserialize, Serialize};

// Comprehensive statistics structures - will be implemented in task 2.4
#[derive(Debug, Serialize, Deserialize)]
pub struct PeakUsageStats {
    pub peak_hour: u8,
    pub peak_day: String,
    pub peak_tokens_per_hour: u32,
    pub peak_cost_per_day: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostEfficiencyStats {
    pub cost_per_token: f64,
    pub cost_per_request: f64,
    pub cache_savings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageTrendStats {
    pub daily_growth_rate: f64,
    pub usage_pattern: String,
    pub usage_percentiles: UsagePercentiles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsagePercentiles {
    pub median: u32,
    pub p95: u32,
    pub p99: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComprehensiveStats {
    pub peak_usage: PeakUsageStats,
    pub cost_efficiency: CostEfficiencyStats,
    pub usage_trends: UsageTrendStats,
}