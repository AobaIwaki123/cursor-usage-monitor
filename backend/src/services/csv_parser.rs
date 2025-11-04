use crate::models::usage_data::UsageData;

// CSV parsing service - will be implemented in task 2.2
pub struct CsvParser;

impl CsvParser {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn parse_csv(&self, _csv_content: &str) -> Result<Vec<UsageData>, String> {
        // Implementation will be added in task 2.2
        Ok(vec![])
    }

    #[allow(dead_code)]
    pub fn validate_csv_format(&self, _csv_content: &str) -> Result<(), String> {
        // Implementation will be added in task 2.2
        Ok(())
    }
}