use crate::models::usage_data::{UsageData, UsageSummary};

// Data processing service - will be implemented in task 2.3
pub struct DataProcessor;

impl DataProcessor {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn calculate_summary(&self, _data: &[UsageData]) -> UsageSummary {
        // Implementation will be added in task 2.3
        todo!("Implementation pending for task 2.3")
    }

    #[allow(dead_code)]
    pub fn merge_data(&self, _existing: Vec<UsageData>, _new: Vec<UsageData>) -> Vec<UsageData> {
        // Implementation will be added in task 2.3
        todo!("Implementation pending for task 2.3")
    }
}