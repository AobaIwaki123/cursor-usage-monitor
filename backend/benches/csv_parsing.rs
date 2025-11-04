use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use cursor_usage_dashboard_backend::services::csv_parser::CsvParser;

fn create_csv_data(num_records: usize) -> String {
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

fn bench_csv_parsing(c: &mut Criterion) {
    let parser = CsvParser::new();
    
    let mut group = c.benchmark_group("csv_parsing");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let csv_data = create_csv_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("parse_csv", size),
            &csv_data,
            |b, csv| {
                b.iter(|| {
                    parser.parse_csv(black_box(csv)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_csv_validation(c: &mut Criterion) {
    let parser = CsvParser::new();
    
    let mut group = c.benchmark_group("csv_validation");
    
    for size in [100, 500, 1000, 5000, 10000].iter() {
        let csv_data = create_csv_data(*size);
        
        group.bench_with_input(
            BenchmarkId::new("validate_csv_format", size),
            &csv_data,
            |b, csv| {
                b.iter(|| {
                    parser.validate_csv_format(black_box(csv)).unwrap()
                })
            },
        );
    }
    
    group.finish();
}

fn bench_large_file_parsing(c: &mut Criterion) {
    let parser = CsvParser::new();
    
    // Test with very large files (simulating up to 100MB)
    let large_csv = create_csv_data(50000); // Approximately 5MB
    
    c.bench_function("parse_large_csv_50k_records", |b| {
        b.iter(|| {
            parser.parse_csv(black_box(&large_csv)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_csv_parsing,
    bench_csv_validation,
    bench_large_file_parsing
);
criterion_main!(benches);