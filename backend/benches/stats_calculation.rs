use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursor_usage_dashboard_backend::{
    models::usage_data::UsageData,
    services::stats_calculator::StatsCalculator,
};

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

fn bench_peak_usage_calculation(c: &mut Criterion) {
    let calculator = StatsCalculator::new();
    
    let mut group = c.benchmark_group("peak_usage_calculation");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let data = create_usage_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("calculate_peak_usage", size),
            &data,
            |b, data| {
                b.iter(|| {
                    calculator.calculate_peak_usage(black_box(data))
                })
            },
        );
    }
    
    group.finish();
}

fn bench_cost_efficiency_calculation(c: &mut Criterion) {
    let calculator = StatsCalculator::new();
    
    let mut group = c.benchmark_group("cost_efficiency_calculation");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let data = create_usage_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("calculate_cost_efficiency", size),
            &data,
            |b, data| {
                b.iter(|| {
                    calculator.calculate_cost_efficiency(black_box(data))
                })
            },
        );
    }
    
    group.finish();
}

fn bench_usage_trends_calculation(c: &mut Criterion) {
    let calculator = StatsCalculator::new();
    
    let mut group = c.benchmark_group("usage_trends_calculation");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let data = create_usage_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("calculate_usage_trends", size),
            &data,
            |b, data| {
                b.iter(|| {
                    calculator.calculate_usage_trends(black_box(data))
                })
            },
        );
    }
    
    group.finish();
}

fn bench_comprehensive_stats_calculation(c: &mut Criterion) {
    let calculator = StatsCalculator::new();
    
    let mut group = c.benchmark_group("comprehensive_stats_calculation");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let data = create_usage_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("calculate_comprehensive_stats", size),
            &data,
            |b, data| {
                b.iter(|| {
                    calculator.calculate_comprehensive_stats(black_box(data))
                })
            },
        );
    }
    
    group.finish();
}

fn bench_large_dataset_stats(c: &mut Criterion) {
    let calculator = StatsCalculator::new();
    
    // Test with very large datasets
    let large_data = create_usage_data(50000);
    
    c.bench_function("comprehensive_stats_50k_records", |b| {
        b.iter(|| {
            calculator.calculate_comprehensive_stats(black_box(&large_data))
        })
    });
}

criterion_group!(
    benches,
    bench_peak_usage_calculation,
    bench_cost_efficiency_calculation,
    bench_usage_trends_calculation,
    bench_comprehensive_stats_calculation,
    bench_large_dataset_stats
);
criterion_main!(benches);