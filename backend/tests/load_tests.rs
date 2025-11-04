use cursor_usage_dashboard_backend::{
    services::{csv_parser::CsvParser, stats_calculator::StatsCalculator},
    storage::UPLOADED_DATA,
};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn create_test_csv_data(num_records: usize) -> String {
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
fn test_concurrent_csv_parsing_load() {
    let parser = Arc::new(CsvParser::new());
    let csv_data = Arc::new(create_test_csv_data(1000));
    
    let num_threads = 10;
    let mut handles = vec![];
    let start_time = Instant::now();
    
    // Spawn multiple threads for concurrent parsing
    for i in 0..num_threads {
        let parser_clone = Arc::clone(&parser);
        let csv_clone = Arc::clone(&csv_data);
        
        let handle = thread::spawn(move || {
            let thread_start = Instant::now();
            
            // Each thread parses the CSV multiple times
            for _ in 0..5 {
                let result = parser_clone.parse_csv(&csv_clone);
                assert!(result.is_ok(), "Thread {} failed to parse CSV", i);
                
                let usage_data = result.unwrap();
                assert_eq!(usage_data.len(), 1000);
            }
            
            let thread_duration = thread_start.elapsed();
            println!("Thread {} completed in {:?}", i, thread_duration);
            thread_duration
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_duration = Duration::new(0, 0);
    for handle in handles {
        let thread_duration = handle.join().unwrap();
        total_duration += thread_duration;
    }
    
    let total_elapsed = start_time.elapsed();
    let avg_thread_duration = total_duration / num_threads as u32;
    
    println!("Load test completed:");
    println!("  Total elapsed time: {:?}", total_elapsed);
    println!("  Average thread duration: {:?}", avg_thread_duration);
    println!("  Total operations: {}", num_threads * 5);
    
    // Performance assertions
    assert!(total_elapsed.as_secs() < 10, "Load test took too long: {:?}", total_elapsed);
    assert!(avg_thread_duration.as_millis() < 1000, "Average thread duration too high: {:?}", avg_thread_duration);
}

#[test]
fn test_concurrent_stats_calculation_load() {
    let calculator = Arc::new(StatsCalculator::new());
    
    // Create test data
    let mut usage_data = Vec::new();
    for i in 0..5000 {
        usage_data.push(cursor_usage_dashboard_backend::models::usage_data::UsageData {
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
    
    let shared_data = Arc::new(usage_data);
    let num_threads = 8;
    let mut handles = vec![];
    let start_time = Instant::now();
    
    // Spawn multiple threads for concurrent stats calculation
    for i in 0..num_threads {
        let calculator_clone = Arc::clone(&calculator);
        let data_clone = Arc::clone(&shared_data);
        
        let handle = thread::spawn(move || {
            let thread_start = Instant::now();
            
            // Each thread calculates stats multiple times
            for _ in 0..3 {
                let stats = calculator_clone.calculate_comprehensive_stats(&data_clone);
                
                // Verify stats are calculated correctly
                assert!(stats.peak_usage.peak_tokens_per_hour > 0);
                assert!(stats.cost_efficiency.cost_per_token > 0.0);
                assert!(!stats.usage_trends.usage_pattern.is_empty());
            }
            
            let thread_duration = thread_start.elapsed();
            println!("Stats thread {} completed in {:?}", i, thread_duration);
            thread_duration
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_duration = Duration::new(0, 0);
    for handle in handles {
        let thread_duration = handle.join().unwrap();
        total_duration += thread_duration;
    }
    
    let total_elapsed = start_time.elapsed();
    let avg_thread_duration = total_duration / num_threads as u32;
    
    println!("Stats load test completed:");
    println!("  Total elapsed time: {:?}", total_elapsed);
    println!("  Average thread duration: {:?}", avg_thread_duration);
    println!("  Total operations: {}", num_threads * 3);
    
    // Performance assertions
    assert!(total_elapsed.as_secs() < 15, "Stats load test took too long: {:?}", total_elapsed);
    assert!(avg_thread_duration.as_millis() < 2000, "Average stats thread duration too high: {:?}", avg_thread_duration);
}

#[test]
fn test_memory_pressure_large_datasets() {
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    // Test with progressively larger datasets to check memory handling
    let sizes = vec![1000, 5000, 10000, 25000, 50000];
    
    for &size in &sizes {
        println!("Testing memory pressure with {} records", size);
        
        let csv_data = create_test_csv_data(size);
        let csv_size = csv_data.len();
        
        let parse_start = Instant::now();
        let result = parser.parse_csv(&csv_data);
        let parse_duration = parse_start.elapsed();
        
        assert!(result.is_ok(), "Failed to parse CSV with {} records", size);
        let usage_data = result.unwrap();
        assert_eq!(usage_data.len(), size);
        
        let stats_start = Instant::now();
        let stats = calculator.calculate_comprehensive_stats(&usage_data);
        let stats_duration = stats_start.elapsed();
        
        // Verify results
        assert!(stats.peak_usage.peak_tokens_per_hour > 0);
        assert!(stats.cost_efficiency.cost_per_token > 0.0);
        
        println!("  CSV size: {} bytes", csv_size);
        println!("  Parse time: {:?}", parse_duration);
        println!("  Stats time: {:?}", stats_duration);
        
        // Performance assertions scale with data size (more realistic thresholds)
        let max_parse_time = Duration::from_millis(size as u64 / 5); // 0.2ms per record
        let max_stats_time = Duration::from_millis(size as u64 / 10); // 0.1ms per record
        
        assert!(parse_duration < max_parse_time, 
                "Parse time {:?} exceeded limit {:?} for {} records", 
                parse_duration, max_parse_time, size);
        assert!(stats_duration < max_stats_time, 
                "Stats time {:?} exceeded limit {:?} for {} records", 
                stats_duration, max_stats_time, size);
    }
}

#[test]
fn test_concurrent_file_upload_simulation() {
    // Clear existing data
    {
        let mut data_store = UPLOADED_DATA.lock().unwrap();
        data_store.clear();
    }
    
    let parser = Arc::new(CsvParser::new());
    let num_concurrent_uploads = 5;
    let records_per_upload = 1000;
    
    let mut handles = vec![];
    let start_time = Instant::now();
    
    // Simulate concurrent file uploads
    for i in 0..num_concurrent_uploads {
        let parser_clone = Arc::clone(&parser);
        
        let handle = thread::spawn(move || {
            let csv_data = create_test_csv_data(records_per_upload);
            let upload_start = Instant::now();
            
            // Parse the CSV (simulating upload processing)
            let result = parser_clone.parse_csv(&csv_data);
            assert!(result.is_ok(), "Upload {} failed to parse", i);
            
            let usage_data = result.unwrap();
            assert_eq!(usage_data.len(), records_per_upload);
            
            let upload_duration = upload_start.elapsed();
            println!("Upload {} completed in {:?}", i, upload_duration);
            
            (i, usage_data.len(), upload_duration)
        });
        
        handles.push(handle);
    }
    
    // Wait for all uploads to complete
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    let total_elapsed = start_time.elapsed();
    let total_records: usize = results.iter().map(|(_, count, _)| count).sum();
    let avg_upload_time: Duration = results.iter()
        .map(|(_, _, duration)| *duration)
        .sum::<Duration>() / num_concurrent_uploads as u32;
    
    println!("Concurrent upload simulation completed:");
    println!("  Total elapsed time: {:?}", total_elapsed);
    println!("  Average upload time: {:?}", avg_upload_time);
    println!("  Total records processed: {}", total_records);
    println!("  Throughput: {:.2} records/sec", total_records as f64 / total_elapsed.as_secs_f64());
    
    // Performance assertions
    assert_eq!(total_records, num_concurrent_uploads * records_per_upload);
    assert!(total_elapsed.as_secs() < 5, "Concurrent uploads took too long: {:?}", total_elapsed);
    assert!(avg_upload_time.as_millis() < 500, "Average upload time too high: {:?}", avg_upload_time);
    
    // Verify throughput is reasonable (at least 1000 records/sec)
    let throughput = total_records as f64 / total_elapsed.as_secs_f64();
    assert!(throughput > 1000.0, "Throughput too low: {:.2} records/sec", throughput);
}

#[test]
fn test_error_handling_under_load() {
    let parser = Arc::new(CsvParser::new());
    let num_threads = 5;
    let mut handles = vec![];
    
    // Test error handling with concurrent invalid data
    for i in 0..num_threads {
        let parser_clone = Arc::clone(&parser);
        
        let handle = thread::spawn(move || {
            // Create invalid CSV data
            let invalid_csv = if i % 2 == 0 {
                // Invalid header
                "Invalid,Headers,Here\n1,2,3".to_string()
            } else {
                // Invalid token calculation
                "Date,Kind,Model,Max Mode,Input (w/ Cache Write),Input (w/o Cache Write),Cache Read,Output Tokens,Total Tokens,Cost\n\
                 2024-01-01T10:00:00Z,Included,auto,No,100,50,25,75,300,0.05".to_string()
            };
            
            let result = parser_clone.parse_csv(&invalid_csv);
            assert!(result.is_err(), "Thread {} should have failed with invalid data", i);
            
            // Verify error message is meaningful
            let error_msg = result.unwrap_err();
            assert!(!error_msg.is_empty(), "Error message should not be empty");
            
            println!("Thread {} correctly handled error: {}", i, error_msg);
            true
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut success_count = 0;
    for handle in handles {
        if handle.join().unwrap() {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, num_threads, "All threads should handle errors correctly");
    println!("Error handling under load test completed successfully");
}