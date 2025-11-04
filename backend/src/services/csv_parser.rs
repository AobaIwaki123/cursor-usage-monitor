use crate::models::usage_data::UsageData;
use csv::ReaderBuilder;
use std::io::Cursor;

pub struct CsvParser;

impl CsvParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_csv(&self, csv_content: &str) -> Result<Vec<UsageData>, String> {
        // First validate the CSV format
        self.validate_csv_format(csv_content)?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_content));

        let mut usage_data = Vec::new();

        for (line_num, result) in reader.records().enumerate() {
            let record = result.map_err(|e| {
                format!("Error reading CSV line {}: {}", line_num + 2, e)
            })?;

            if record.len() != 10 {
                return Err(format!(
                    "Invalid number of columns on line {}. Expected 10, found {}",
                    line_num + 2,
                    record.len()
                ));
            }

            let usage_entry = self.parse_record(&record, line_num + 2)?;
            usage_data.push(usage_entry);
        }

        if usage_data.is_empty() {
            return Err("CSV file contains no data rows".to_string());
        }

        Ok(usage_data)
    }

    pub fn validate_csv_format(&self, csv_content: &str) -> Result<(), String> {
        if csv_content.trim().is_empty() {
            return Err("CSV file is empty".to_string());
        }

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(Cursor::new(csv_content));

        // Check headers
        let headers = reader.headers().map_err(|e| {
            format!("Error reading CSV headers: {}", e)
        })?;

        let expected_headers = [
            "Date",
            "Kind", 
            "Model",
            "Max Mode",
            "Input (w/ Cache Write)",
            "Input (w/o Cache Write)",
            "Cache Read",
            "Output Tokens",
            "Total Tokens",
            "Cost"
        ];

        if headers.len() != expected_headers.len() {
            return Err(format!(
                "Invalid number of columns in header. Expected {}, found {}",
                expected_headers.len(),
                headers.len()
            ));
        }

        for (i, expected) in expected_headers.iter().enumerate() {
            let actual = headers.get(i).unwrap_or("");
            if actual != *expected {
                return Err(format!(
                    "Invalid header at column {}. Expected '{}', found '{}'",
                    i + 1,
                    expected,
                    actual
                ));
            }
        }

        Ok(())
    }

    fn parse_record(&self, record: &csv::StringRecord, line_num: usize) -> Result<UsageData, String> {
        let date = record.get(0).unwrap_or("").trim_matches('"').to_string();
        if date.is_empty() {
            return Err(format!("Empty date field on line {}", line_num));
        }

        let kind = record.get(1).unwrap_or("").trim_matches('"').to_string();
        let model = record.get(2).unwrap_or("").trim_matches('"').to_string();
        
        let max_mode_str = record.get(3).unwrap_or("").trim_matches('"');
        let max_mode = match max_mode_str.to_lowercase().as_str() {
            "yes" | "true" | "1" => true,
            "no" | "false" | "0" => false,
            _ => return Err(format!("Invalid Max Mode value '{}' on line {}. Expected 'Yes' or 'No'", max_mode_str, line_num)),
        };

        let input_with_cache = self.parse_u32_field(record.get(4), "Input (w/ Cache Write)", line_num)?;
        let input_without_cache = self.parse_u32_field(record.get(5), "Input (w/o Cache Write)", line_num)?;
        let cache_read = self.parse_u32_field(record.get(6), "Cache Read", line_num)?;
        let output_tokens = self.parse_u32_field(record.get(7), "Output Tokens", line_num)?;
        let total_tokens = self.parse_u32_field(record.get(8), "Total Tokens", line_num)?;
        let cost = self.parse_f64_field(record.get(9), "Cost", line_num)?;

        // Validate data consistency
        let calculated_total = input_with_cache + input_without_cache + cache_read + output_tokens;
        if calculated_total != total_tokens {
            return Err(format!(
                "Token calculation mismatch on line {}. Sum of individual tokens ({}) doesn't match Total Tokens ({})",
                line_num, calculated_total, total_tokens
            ));
        }

        Ok(UsageData {
            date,
            kind,
            model,
            max_mode,
            input_with_cache,
            input_without_cache,
            cache_read,
            output_tokens,
            total_tokens,
            cost,
        })
    }

    fn parse_u32_field(&self, field: Option<&str>, field_name: &str, line_num: usize) -> Result<u32, String> {
        let value_str = field.unwrap_or("").trim_matches('"');
        value_str.parse::<u32>().map_err(|_| {
            format!("Invalid {} value '{}' on line {}. Expected a positive integer", field_name, value_str, line_num)
        })
    }

    fn parse_f64_field(&self, field: Option<&str>, field_name: &str, line_num: usize) -> Result<f64, String> {
        let value_str = field.unwrap_or("").trim_matches('"');
        value_str.parse::<f64>().map_err(|_| {
            format!("Invalid {} value '{}' on line {}. Expected a number", field_name, value_str, line_num)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_csv() -> String {
        "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
         2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,250,0.05\n\
         2024-01-01T11:00:00Z,Included,gpt-4,Yes,200,100,50,150,500,0.15".to_string()
    }

    fn create_invalid_header_csv() -> String {
        "Date,Kind,Model,Invalid Header,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
         2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,250,0.05".to_string()
    }

    #[test]
    fn test_parse_valid_csv() {
        let parser = CsvParser::new();
        let csv_content = create_valid_csv();
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_ok());
        
        let data = result.unwrap();
        assert_eq!(data.len(), 2);
        
        // Check first record
        assert_eq!(data[0].date, "2024-01-01T10:00:00Z");
        assert_eq!(data[0].kind, "Included");
        assert_eq!(data[0].model, "auto");
        assert_eq!(data[0].max_mode, false);
        assert_eq!(data[0].input_with_cache, 100);
        assert_eq!(data[0].input_without_cache, 50);
        assert_eq!(data[0].cache_read, 25);
        assert_eq!(data[0].output_tokens, 75);
        assert_eq!(data[0].total_tokens, 250);
        assert_eq!(data[0].cost, 0.05);
    }

    #[test]
    fn test_parse_empty_csv() {
        let parser = CsvParser::new();
        let result = parser.parse_csv("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("CSV file is empty"));
    }

    #[test]
    fn test_parse_invalid_headers() {
        let parser = CsvParser::new();
        let csv_content = create_invalid_header_csv();
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid header at column 4"));
    }

    #[test]
    fn test_parse_invalid_token_calculation() {
        let parser = CsvParser::new();
        let csv_content = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                          2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,300,0.05";
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Token calculation mismatch"));
    }

    #[test]
    fn test_parse_invalid_number_format() {
        let parser = CsvParser::new();
        let csv_content = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                          2024-01-01T10:00:00Z,Included,auto,No,invalid,50,25,75,250,0.05";
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Input (w/ Cache Write) value"));
    }

    #[test]
    fn test_parse_invalid_max_mode() {
        let parser = CsvParser::new();
        let csv_content = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                          2024-01-01T10:00:00Z,Included,auto,Maybe,100,50,25,75,250,0.05";
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid Max Mode value"));
    }

    #[test]
    fn test_parse_wrong_column_count() {
        let parser = CsvParser::new();
        let csv_content = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                          2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75";
        
        let result = parser.parse_csv(&csv_content);
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        // The CSV library returns different error messages, so we check for various possibilities
        assert!(error_msg.contains("found record with 8 fields") || 
                error_msg.contains("Invalid number of columns") || 
                error_msg.contains("Expected 10, found"));
    }

    #[test]
    fn test_validate_csv_format_success() {
        let parser = CsvParser::new();
        let csv_content = create_valid_csv();
        
        let result = parser.validate_csv_format(&csv_content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_csv_format_empty() {
        let parser = CsvParser::new();
        let result = parser.validate_csv_format("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("CSV file is empty"));
    }

    #[test]
    fn test_validate_csv_format_wrong_header_count() {
        let parser = CsvParser::new();
        let csv_content = "Date,Kind,Model\n2024-01-01,Included,auto";
        
        let result = parser.validate_csv_format(&csv_content);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid number of columns in header"));
    }
}