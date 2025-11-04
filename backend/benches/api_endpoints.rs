use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use axum::Router;
use axum_test::TestServer;
use cursor_usage_dashboard_backend::{
    handlers::{health, upload, stats},
    storage::UPLOADED_DATA,
};
use tower_http::cors::CorsLayer;

fn create_test_app() -> Router {
    Router::new()
        .route("/api/health", axum::routing::get(health::health_check))
        .route("/api/upload", axum::routing::post(upload::upload_csv))
        .route("/api/upload/append", axum::routing::post(upload::append_csv))
        .route("/api/stats/comprehensive", axum::routing::get(stats::comprehensive_stats))
        .layer(CorsLayer::permissive())
}

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

fn create_multipart_body(csv_content: &str, filename: &str) -> String {
    let boundary = "----formdata-test-boundary";
    format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"csvFile\"; filename=\"{filename}\"\r\n\
         Content-Type: text/csv\r\n\
         \r\n\
         {csv_content}\r\n\
         --{boundary}--\r\n",
        boundary = boundary,
        filename = filename,
        csv_content = csv_content
    )
}

fn bench_health_endpoint(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("health_endpoint", |b| {
        b.iter(|| {
            rt.block_on(async {
                let app = create_test_app();
                let server = TestServer::new(app).unwrap();
                let response = server.get("/api/health").await;
                black_box(response.status_code())
            })
        })
    });
}

fn bench_upload_endpoint(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("upload_endpoint");
    
    for size in [10, 50, 100, 500, 1000].iter() {
        let csv_data = create_csv_data(*size);
        let multipart_body = create_multipart_body(&csv_data, "test.csv");
        
        group.bench_with_input(
            BenchmarkId::new("upload_csv", size),
            &multipart_body,
            |b, body| {
                b.iter(|| {
                    let body_clone = body.clone();
                    rt.block_on(async {
                        let app = create_test_app();
                        let server = TestServer::new(app).unwrap();
                        
                        // Clear existing data
                        {
                            let mut data_store = UPLOADED_DATA.lock().unwrap();
                            data_store.clear();
                        }
                        
                        let response = server
                            .post("/api/upload")
                            .content_type("multipart/form-data; boundary=----formdata-test-boundary")
                            .text(body_clone)
                            .await;
                        
                        black_box(response.status_code())
                    })
                })
            },
        );
    }
    
    group.finish();
}

fn bench_stats_endpoint(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("comprehensive_stats_endpoint", |b| {
        b.iter(|| {
            rt.block_on(async {
                let app = create_test_app();
                let server = TestServer::new(app).unwrap();

                // Upload test data first
                let csv_data = create_csv_data(1000);
                let multipart_body = create_multipart_body(&csv_data, "test.csv");
                
                let _response = server
                    .post("/api/upload")
                    .content_type("multipart/form-data; boundary=----formdata-test-boundary")
                    .text(multipart_body)
                    .await;

                let response = server.get("/api/stats/comprehensive").await;
                black_box(response.status_code())
            })
        })
    });
}

fn bench_concurrent_requests(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("concurrent_health_requests", |b| {
        b.iter(|| {
            rt.block_on(async {
                let app = create_test_app();
                let server = TestServer::new(app).unwrap();
                
                // Make 10 sequential requests to simulate concurrent load
                let mut results = vec![];
                for _ in 0..10 {
                    let response = server.get("/api/health").await;
                    results.push(response.status_code());
                }
                
                black_box(results)
            })
        })
    });
}

fn bench_memory_usage_large_upload(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Test memory efficiency with large files
    let large_csv = create_csv_data(10000); // Large dataset
    let multipart_body = create_multipart_body(&large_csv, "large.csv");
    
    c.bench_function("large_file_upload_10k_records", |b| {
        b.iter(|| {
            let body_clone = multipart_body.clone();
            rt.block_on(async {
                let app = create_test_app();
                let server = TestServer::new(app).unwrap();
                
                // Clear existing data
                {
                    let mut data_store = UPLOADED_DATA.lock().unwrap();
                    data_store.clear();
                }
                
                let response = server
                    .post("/api/upload")
                    .content_type("multipart/form-data; boundary=----formdata-test-boundary")
                    .text(body_clone)
                    .await;
                
                black_box(response.status_code())
            })
        })
    });
}

criterion_group!(
    benches,
    bench_health_endpoint,
    bench_upload_endpoint,
    bench_stats_endpoint,
    bench_concurrent_requests,
    bench_memory_usage_large_upload
);
criterion_main!(benches);