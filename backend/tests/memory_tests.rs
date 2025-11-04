use cursor_usage_dashboard_backend::{
    models::usage_data::UsageData,
    services::{csv_parser::CsvParser, stats_calculator::StatsCalculator},
};
use std::mem;

fn create_large_csv_data(num_records: usize) -> String {
    let mut csv = "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n".to_string();
    
    for i in 0..num_records {
        csv.push_str(&format!(
            "2024-01-{:02}T{:02}:00:00Z,Included,auto,No,100,50,25,75,250,0.05\n",
            (i % 30) + 1,
            (i % 24)
        ));
    }
    
    csv
}

#[test]
fn test_memory_usage_large_csv_parsing() {
    let parser = CsvParser::new();
    
    // Test with different dataset sizes
    for &size in &[1000, 5000, 10000, 25000] {
        let csv_data = create_large_csv_data(size);
        let csv_size = csv_data.len();
        
        println!("Testing CSV parsing with {} records, CSV size: {} bytes", size, csv_size);
        
        let result = parser.parse_csv(&csv_data);
        assert!(result.is_ok());
        
        let usage_data = result.unwrap();
        assert_eq!(usage_data.len(), size);
        
        // Calculate memory usage of parsed data
        let data_memory = usage_data.len() * mem::size_of::<UsageData>();
        println!("Parsed data memory usage: {} bytes", data_memory);
        
        // Memory efficiency check: parsed data should not be excessively larger than CSV
        let memory_ratio = data_memory as f64 / csv_size as f64;
        println!("Memory ratio (parsed/csv): {:.2}", memory_ratio);
        
        // The parsed data should be reasonable compared to CSV size
        // This is a rough check - in practice, structured data might be larger
        assert!(memory_ratio < 10.0, "Memory usage too high for {} records", size);
    }
}

#[test]
fn test_memory_usage_stats_calculation() {
    let calculator = StatsCalculator::new();
    
    // Create test data
    let mut usage_data = Vec::new();
    for i in 0..10000 {
        usage_data.push(UsageData {
            date: format!("2024-01-{:02}T{:02}:00:00Z", (i % 30) + 1, i % 24),
            kind: "Included".to_string(),
            model: if i % 3 == 0 { "auto".to_string() } else { "gpt-4".to_string() },
            max_mode: i % 2 == 0,
            input_with_cache: 100 + (i % 100) as u32,
            input_without_cache: 50 + (i % 50) as u32,
            cache_read: 25 + (i % 25) as u32,
            output_tokens: 75 + (i % 75) as u32,
            total_tokens: 250 + (i % 250) as u32,
            cost: 0.05 + (i as f64 * 0.001),
        });
    }
    
    let data_memory = usage_data.len() * mem::size_of::<UsageData>();
    println!("Input data memory usage: {} bytes", data_memory);
    
    // Test comprehensive stats calculation
    let stats = calculator.calculate_comprehensive_stats(&usage_data);
    
    // Verify stats are calculated correctly
    assert!(stats.peak_usage.peak_tokens_per_hour > 0);
    assert!(stats.cost_efficiency.cost_per_token > 0.0);
    assert!(!stats.usage_trends.usage_pattern.is_empty());
    
    println!("Stats calculation completed successfully for {} records", usage_data.len());
}

#[test]
fn test_concurrent_memory_usage() {
    use std::sync::Arc;
    use std::thread;
    
    let parser = Arc::new(CsvParser::new());
    let csv_data = Arc::new(create_large_csv_data(1000));
    
    let mut handles = vec![];
    
    // Spawn multiple threads to test concurrent parsing
    for i in 0..5 {
        let parser_clone = Arc::clone(&parser);
        let csv_clone = Arc::clone(&csv_data);
        
        let handle = thread::spawn(move || {
            println!("Thread {} starting CSV parsing", i);
            let result = parser_clone.parse_csv(&csv_clone);
            assert!(result.is_ok());
            let data = result.unwrap();
            println!("Thread {} completed parsing {} records", i, data.len());
            data.len()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_records = 0;
    for handle in handles {
        total_records += handle.join().unwrap();
    }
    
    println!("Total records processed across all threads: {}", total_records);
    assert_eq!(total_records, 5000); // 5 threads * 1000 records each
}

#[test]
fn test_large_dataset_processing() {
    // Test processing of very large datasets (simulating 100MB file)
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    // Create a dataset that would represent a large CSV file
    let large_csv = create_large_csv_data(50000); // Approximately 5MB
    
    println!("Processing large CSV with {} bytes", large_csv.len());
    
    // Parse the large CSV
    let start_time = std::time::Instant::now();
    let result = parser.parse_csv(&large_csv);
    let parse_duration = start_time.elapsed();
    
    assert!(result.is_ok());
    let usage_data = result.unwrap();
    
    println!("Parsed {} records in {:?}", usage_data.len(), parse_duration);
    
    // Calculate comprehensive stats
    let stats_start = std::time::Instant::now();
    let stats = calculator.calculate_comprehensive_stats(&usage_data);
    let stats_duration = stats_start.elapsed();
    
    println!("Calculated stats in {:?}", stats_duration);
    
    // Verify results
    assert_eq!(usage_data.len(), 50000);
    assert!(stats.peak_usage.peak_tokens_per_hour > 0);
    assert!(stats.cost_efficiency.cost_per_token > 0.0);
    
    // Performance assertions (these are rough guidelines)
    assert!(parse_duration.as_secs() < 5, "Parsing took too long: {:?}", parse_duration);
    assert!(stats_duration.as_secs() < 2, "Stats calculation took too long: {:?}", stats_duration);
}