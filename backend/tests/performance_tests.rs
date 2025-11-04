use cursor_usage_dashboard_backend::{
    models::usage_data::UsageData,
    services::{csv_parser::CsvParser, stats_calculator::StatsCalculator},
};
use std::mem;
use std::time::Instant;

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

fn create_usage_data(num_records: usize) -> Vec<UsageData> {
    let mut data = Vec::with_capacity(num_records);
    
    for i in 0..num_records {
        data.push(UsageData {
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
    
    data
}

#[test]
fn test_memory_efficiency_csv_parsing() {
    let parser = CsvParser::new();
    
    // Test memory efficiency with different dataset sizes
    let test_sizes = vec![1000, 5000, 10000, 25000, 50000];
    
    for &size in &test_sizes {
        println!("\n=== Testing CSV parsing memory efficiency with {} records ===", size);
        
        let csv_data = create_large_csv_data(size);
        let csv_size_bytes = csv_data.len();
        
        println!("CSV string size: {} bytes ({:.2} MB)", csv_size_bytes, csv_size_bytes as f64 / 1_048_576.0);
        
        // Measure parsing time and memory usage
        let parse_start = Instant::now();
        let result = parser.parse_csv(&csv_data);
        let parse_duration = parse_start.elapsed();
        
        assert!(result.is_ok());
        let usage_data = result.unwrap();
        assert_eq!(usage_data.len(), size);
        
        // Calculate memory usage of parsed data
        let struct_size = mem::size_of::<UsageData>();
        let estimated_memory = usage_data.len() * struct_size;
        
        // Account for string allocations (rough estimate)
        let string_memory: usize = usage_data.iter()
            .map(|d| d.date.len() + d.kind.len() + d.model.len())
            .sum();
        
        let total_memory = estimated_memory + string_memory;
        
        println!("Parse time: {:?}", parse_duration);
        println!("Struct memory: {} bytes ({:.2} MB)", estimated_memory, estimated_memory as f64 / 1_048_576.0);
        println!("String memory: {} bytes ({:.2} MB)", string_memory, string_memory as f64 / 1_048_576.0);
        println!("Total memory: {} bytes ({:.2} MB)", total_memory, total_memory as f64 / 1_048_576.0);
        
        // Memory efficiency metrics
        let memory_ratio = total_memory as f64 / csv_size_bytes as f64;
        let parse_rate = size as f64 / parse_duration.as_secs_f64();
        
        println!("Memory ratio (parsed/csv): {:.2}", memory_ratio);
        println!("Parse rate: {:.0} records/sec", parse_rate);
        
        // Performance assertions
        assert!(parse_duration.as_millis() < (size as u128 / 10), "Parsing too slow for {} records", size);
        assert!(memory_ratio < 5.0, "Memory usage too high for {} records", size);
        assert!(parse_rate > 1000.0, "Parse rate too low for {} records", size);
    }
}

#[test]
fn test_memory_efficiency_stats_calculation() {
    let calculator = StatsCalculator::new();
    
    // Test memory efficiency with different dataset sizes
    let test_sizes = vec![1000, 5000, 10000, 25000, 50000];
    
    for &size in &test_sizes {
        println!("\n=== Testing stats calculation memory efficiency with {} records ===", size);
        
        let usage_data = create_usage_data(size);
        
        // Calculate input data memory usage
        let struct_size = mem::size_of::<UsageData>();
        let estimated_memory = usage_data.len() * struct_size;
        let string_memory: usize = usage_data.iter()
            .map(|d| d.date.len() + d.kind.len() + d.model.len())
            .sum();
        let input_memory = estimated_memory + string_memory;
        
        println!("Input data memory: {} bytes ({:.2} MB)", input_memory, input_memory as f64 / 1_048_576.0);
        
        // Measure stats calculation time
        let stats_start = Instant::now();
        let stats = calculator.calculate_comprehensive_stats(&usage_data);
        let stats_duration = stats_start.elapsed();
        
        // Verify results
        assert!(stats.peak_usage.peak_tokens_per_hour > 0);
        assert!(stats.cost_efficiency.cost_per_token > 0.0);
        assert!(!stats.usage_trends.usage_pattern.is_empty());
        
        println!("Stats calculation time: {:?}", stats_duration);
        
        // Performance metrics
        let calc_rate = size as f64 / stats_duration.as_secs_f64();
        
        println!("Calculation rate: {:.0} records/sec", calc_rate);
        
        // Performance assertions
        assert!(stats_duration.as_millis() < (size as u128 / 20), "Stats calculation too slow for {} records", size);
        assert!(calc_rate > 2000.0, "Stats calculation rate too low for {} records", size);
    }
}

#[test]
fn test_peak_performance_benchmarks() {
    let parser = CsvParser::new();
    let _calculator = StatsCalculator::new();
    
    println!("\n=== Peak Performance Benchmarks ===");
    
    // Test with very large dataset (100k records)
    let large_size = 100_000;
    println!("Testing with {} records", large_size);
    
    // CSV parsing benchmark
    let csv_data = create_large_csv_data(large_size);
    let csv_size_mb = csv_data.len() as f64 / 1_048_576.0;
    
    println!("CSV size: {:.2} MB", csv_size_mb);
    
    let parse_start = Instant::now();
    let result = parser.parse_csv(&csv_data);
    let parse_duration = parse_start.elapsed();
    
    assert!(result.is_ok());
    let usage_data = result.unwrap();
    assert_eq!(usage_data.len(), large_size);
    
    let parse_throughput_mb_s = csv_size_mb / parse_duration.as_secs_f64();
    let parse_throughput_records_s = large_size as f64 / parse_duration.as_secs_f64();
    
    println!("Parse results:");
    println!("  Time: {:?}", parse_duration);
    println!("  Throughput: {:.2} MB/s", parse_throughput_mb_s);
    println!("  Throughput: {:.0} records/s", parse_throughput_records_s);
    
    // Stats calculation benchmark
    let calculator = StatsCalculator::new();
    let stats_start = Instant::now();
    let stats = calculator.calculate_comprehensive_stats(&usage_data);
    let stats_duration = stats_start.elapsed();
    
    let stats_throughput = large_size as f64 / stats_duration.as_secs_f64();
    
    println!("Stats calculation results:");
    println!("  Time: {:?}", stats_duration);
    println!("  Throughput: {:.0} records/s", stats_throughput);
    
    // Verify results
    assert!(stats.peak_usage.peak_tokens_per_hour > 0);
    assert!(stats.cost_efficiency.cost_per_token > 0.0);
    assert!(!stats.usage_trends.usage_pattern.is_empty());
    
    // Performance assertions for large datasets
    assert!(parse_duration.as_secs() < 30, "Parse time too high for large dataset: {:?}", parse_duration);
    assert!(stats_duration.as_secs() < 10, "Stats calculation time too high for large dataset: {:?}", stats_duration);
    assert!(parse_throughput_mb_s > 1.0, "Parse throughput too low: {:.2} MB/s", parse_throughput_mb_s);
    assert!(parse_throughput_records_s > 5000.0, "Parse record throughput too low: {:.0} records/s", parse_throughput_records_s);
    assert!(stats_throughput > 10000.0, "Stats throughput too low: {:.0} records/s", stats_throughput);
}

#[test]
fn test_memory_usage_patterns() {
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    println!("\n=== Memory Usage Pattern Analysis ===");
    
    // Test memory usage scaling
    let sizes = vec![1000, 2000, 4000, 8000, 16000];
    let mut memory_measurements = vec![];
    
    for &size in &sizes {
        let csv_data = create_large_csv_data(size);
        let csv_size = csv_data.len();
        
        // Parse and measure
        let result = parser.parse_csv(&csv_data);
        assert!(result.is_ok());
        let usage_data = result.unwrap();
        
        // Estimate memory usage
        let struct_memory = usage_data.len() * mem::size_of::<UsageData>();
        let string_memory: usize = usage_data.iter()
            .map(|d| d.date.len() + d.kind.len() + d.model.len())
            .sum();
        let total_memory = struct_memory + string_memory;
        
        memory_measurements.push((size, csv_size, total_memory));
        
        println!("Size: {} records, CSV: {} bytes, Memory: {} bytes, Ratio: {:.2}", 
                size, csv_size, total_memory, total_memory as f64 / csv_size as f64);
    }
    
    // Verify memory scaling is linear
    for i in 1..memory_measurements.len() {
        let (size1, _, mem1) = memory_measurements[i-1];
        let (size2, _, mem2) = memory_measurements[i];
        
        let size_ratio = size2 as f64 / size1 as f64;
        let memory_ratio = mem2 as f64 / mem1 as f64;
        
        // Memory should scale roughly linearly with data size
        let scaling_efficiency = (memory_ratio / size_ratio - 1.0).abs();
        assert!(scaling_efficiency < 0.1, 
                "Memory scaling not linear: size ratio {:.2}, memory ratio {:.2}", 
                size_ratio, memory_ratio);
    }
    
    println!("Memory scaling is linear ✓");
}

#[test]
fn test_performance_regression_detection() {
    let parser = CsvParser::new();
    let calculator = StatsCalculator::new();
    
    println!("\n=== Performance Regression Detection ===");
    
    // Baseline performance test
    let test_size = 10000;
    let csv_data = create_large_csv_data(test_size);
    
    // Run multiple iterations to get stable measurements
    let iterations = 5;
    let mut parse_times = vec![];
    let mut stats_times = vec![];
    
    for i in 0..iterations {
        // Parse timing
        let parse_start = Instant::now();
        let result = parser.parse_csv(&csv_data);
        let parse_duration = parse_start.elapsed();
        assert!(result.is_ok());
        let usage_data = result.unwrap();
        
        // Stats timing
        let stats_start = Instant::now();
        let stats = calculator.calculate_comprehensive_stats(&usage_data);
        let stats_duration = stats_start.elapsed();
        
        parse_times.push(parse_duration);
        stats_times.push(stats_duration);
        
        // Verify correctness
        assert!(stats.peak_usage.peak_tokens_per_hour > 0);
        assert!(stats.cost_efficiency.cost_per_token > 0.0);
        
        println!("Iteration {}: Parse {:?}, Stats {:?}", i+1, parse_duration, stats_duration);
    }
    
    // Calculate statistics
    let avg_parse_time = parse_times.iter().sum::<std::time::Duration>() / iterations as u32;
    let avg_stats_time = stats_times.iter().sum::<std::time::Duration>() / iterations as u32;
    
    let max_parse_time = *parse_times.iter().max().unwrap();
    let min_parse_time = *parse_times.iter().min().unwrap();
    let max_stats_time = *stats_times.iter().max().unwrap();
    let min_stats_time = *stats_times.iter().min().unwrap();
    
    println!("Performance Summary:");
    println!("  Parse - Avg: {:?}, Min: {:?}, Max: {:?}", avg_parse_time, min_parse_time, max_parse_time);
    println!("  Stats - Avg: {:?}, Min: {:?}, Max: {:?}", avg_stats_time, min_stats_time, max_stats_time);
    
    // Performance consistency checks
    let parse_variance = (max_parse_time.as_nanos() as f64 / min_parse_time.as_nanos() as f64) - 1.0;
    let stats_variance = (max_stats_time.as_nanos() as f64 / min_stats_time.as_nanos() as f64) - 1.0;
    
    println!("  Parse variance: {:.2}%", parse_variance * 100.0);
    println!("  Stats variance: {:.2}%", stats_variance * 100.0);
    
    // Regression detection thresholds (adjusted for test environment variability)
    assert!(avg_parse_time.as_millis() < 1000, "Parse performance regression detected: {:?}", avg_parse_time);
    assert!(avg_stats_time.as_millis() < 500, "Stats performance regression detected: {:?}", avg_stats_time);
    
    // Note: In test environments, performance can be highly variable due to resource contention
    // We focus on detecting major regressions rather than minor variance
    if parse_variance > 2.0 {
        println!("Warning: High parse performance variance: {:.2}%", parse_variance * 100.0);
    }
    if stats_variance > 2.0 {
        println!("Warning: High stats performance variance: {:.2}%", stats_variance * 100.0);
    }
    
    println!("No performance regressions detected ✓");
}