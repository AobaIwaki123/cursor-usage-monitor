use cursor_usage_dashboard_backend::{
    services::{csv_parser::CsvParser, stats_calculator::StatsCalculator},
};

fn create_test_csv_data() -> String {
    "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
     2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,250,0.05\n\
     2024-01-01T11:00:00Z,Included,gpt-4,Yes,200,100,50,150,500,0.15".to_string()
}

#[test]
fn test_end_to_end_csv_processing() {
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    // Parse CSV data
    let csv_content = create_test_csv_data();
    let result = parser.parse_csv(&csv_content);
    assert!(result.is_ok());
    
    let usage_data = result.unwrap();
    assert_eq!(usage_data.len(), 2);
    
    // Calculate comprehensive stats
    let stats = calculator.calculate_comprehensive_stats(&usage_data);
    
    // Verify stats are calculated
    assert!(stats.peak_usage.peak_tokens_per_hour > 0);
    assert!(stats.cost_efficiency.cost_per_token > 0.0);
    assert!(!stats.usage_trends.usage_pattern.is_empty());
    
    println!("End-to-end test completed successfully");
    println!("Peak usage hour: {}", stats.peak_usage.peak_hour);
    println!("Cost per token: {:.6}", stats.cost_efficiency.cost_per_token);
    println!("Usage pattern: {}", stats.usage_trends.usage_pattern);
}

#[test]
fn test_error_handling_workflow() {
    let parser = CsvParser::new();
    
    // Test invalid CSV format
    let invalid_csv = "Invalid,CSV,Format\n1,2,3";
    let result = parser.parse_csv(&invalid_csv);
    assert!(result.is_err());
    
    // Test empty CSV
    let empty_csv = "";
    let result = parser.parse_csv(&empty_csv);
    assert!(result.is_err());
    
    // Test CSV with wrong column count
    let wrong_columns = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                        2024-01-01T10:00:00Z,Included,auto,No,100,50,25";
    let result = parser.parse_csv(&wrong_columns);
    assert!(result.is_err());
    
    println!("Error handling tests completed successfully");
}

#[test]
fn test_large_dataset_processing() {
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    // Create a large CSV dataset
    let mut large_csv = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n".to_string();
    
    for i in 0..1000 {
        large_csv.push_str(&format!(
            "2024-01-{:02}T{:02}:00:00Z,Included,auto,No,100,50,25,75,250,0.05\n",
            (i % 30) + 1,
            (i % 24)
        ));
    }
    
    let start_time = std::time::Instant::now();
    
    // Parse large dataset
    let result = parser.parse_csv(&large_csv);
    assert!(result.is_ok());
    
    let usage_data = result.unwrap();
    assert_eq!(usage_data.len(), 1000);
    
    let parse_duration = start_time.elapsed();
    
    // Calculate stats for large dataset
    let stats_start = std::time::Instant::now();
    let stats = calculator.calculate_comprehensive_stats(&usage_data);
    let stats_duration = stats_start.elapsed();
    
    // Verify results
    assert!(stats.peak_usage.peak_tokens_per_hour > 0);
    assert!(stats.cost_efficiency.cost_per_token > 0.0);
    
    println!("Large dataset test completed successfully");
    println!("Parse time for 1000 records: {:?}", parse_duration);
    println!("Stats calculation time: {:?}", stats_duration);
    
    // Performance assertions (reasonable limits)
    assert!(parse_duration.as_millis() < 1000, "Parsing took too long");
    assert!(stats_duration.as_millis() < 500, "Stats calculation took too long");
}

#[test]
fn test_data_validation_edge_cases() {
    let parser = CsvParser::new();
    
    // Test with negative numbers (should fail)
    let negative_csv = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                       2024-01-01T10:00:00Z,Included,auto,No,-100,50,25,75,250,0.05";
    let result = parser.parse_csv(&negative_csv);
    assert!(result.is_err());
    
    // Test with invalid date format
    let invalid_date_csv = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                          invalid-date,Included,auto,No,100,50,25,75,250,0.05";
    let result = parser.parse_csv(&invalid_date_csv);
    // This should still parse (we don't validate date format in parsing, only in stats calculation)
    assert!(result.is_ok());
    
    // Test with token calculation mismatch
    let mismatch_csv = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                       2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,300,0.05";
    let result = parser.parse_csv(&mismatch_csv);
    assert!(result.is_err());
    
    println!("Data validation edge cases completed successfully");
}

#[test]
fn test_concurrent_processing() {
    use std::sync::Arc;
    use std::thread;
    
    let parser = Arc::new(CsvParser::new());
    let calculator = Arc::new(StatsCalculator::new());
    let csv_data = Arc::new(create_test_csv_data());
    
    let mut handles = vec![];
    
    // Spawn multiple threads for concurrent processing
    for i in 0..5 {
        let parser_clone = Arc::clone(&parser);
        let calculator_clone = Arc::clone(&calculator);
        let csv_clone = Arc::clone(&csv_data);
        
        let handle = thread::spawn(move || {
            // Parse CSV
            let result = parser_clone.parse_csv(&csv_clone);
            assert!(result.is_ok());
            
            let usage_data = result.unwrap();
            
            // Calculate stats
            let stats = calculator_clone.calculate_comprehensive_stats(&usage_data);
            
            println!("Thread {} completed processing", i);
            (usage_data.len(), stats.peak_usage.peak_hour)
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // Verify all threads processed the same data correctly
    for (record_count, peak_hour) in results {
        assert_eq!(record_count, 2);
        assert!(peak_hour <= 23); // Valid hour
    }
    
    println!("Concurrent processing test completed successfully");
}