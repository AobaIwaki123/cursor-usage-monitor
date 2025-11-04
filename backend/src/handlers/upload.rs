use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::models::{
    usage_data::{UsageData, UsageSummary, ModelStats, DateRange},
    error::{ErrorResponse, ErrorDetails},
};
use crate::services::csv_parser::CsvParser;

// In-memory storage for uploaded data (will be replaced with proper storage in production)
lazy_static::lazy_static! {
    static ref UPLOADED_DATA: Arc<Mutex<Vec<UsageData>>> = Arc::new(Mutex::new(Vec::new()));
}

const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB

pub async fn upload_csv(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut csv_content = String::new();
    let mut file_received = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "MULTIPART_ERROR",
            &format!("Error processing multipart data: {}", e),
        )
    })? {
        let field_name = field.name().unwrap_or("");
        
        if field_name == "csvFile" {
            let file_name = field.file_name().unwrap_or("unknown").to_string();
            
            // Validate file extension
            if !file_name.to_lowercase().ends_with(".csv") {
                return Err(create_error_response(
                    StatusCode::BAD_REQUEST,
                    "INVALID_FILE_TYPE",
                    "Only CSV files are allowed",
                ));
            }

            let data = field.bytes().await.map_err(|e| {
                create_error_response(
                    StatusCode::BAD_REQUEST,
                    "FILE_READ_ERROR",
                    &format!("Error reading file: {}", e),
                )
            })?;

            // Check file size
            if data.len() > MAX_FILE_SIZE {
                return Err(create_error_response(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "FILE_TOO_LARGE",
                    &format!("File size ({} bytes) exceeds maximum allowed size ({} bytes)", data.len(), MAX_FILE_SIZE),
                ));
            }

            csv_content = String::from_utf8(data.to_vec()).map_err(|_| {
                create_error_response(
                    StatusCode::BAD_REQUEST,
                    "INVALID_ENCODING",
                    "File must be UTF-8 encoded",
                )
            })?;

            file_received = true;
            break;
        }
    }

    if !file_received {
        return Err(create_error_response(
            StatusCode::BAD_REQUEST,
            "NO_FILE",
            "No CSV file provided. Please upload a file with field name 'csvFile'",
        ));
    }

    // Parse CSV content
    let parser = CsvParser::new();
    let usage_data = parser.parse_csv(&csv_content).map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "CSV_PARSE_ERROR",
            &e,
        )
    })?;

    // Store data in memory (replace existing data)
    {
        let mut data_store = UPLOADED_DATA.lock().unwrap();
        *data_store = usage_data.clone();
    }

    // Calculate summary
    let summary = calculate_summary(&usage_data);

    Ok(Json(json!({
        "success": true,
        "message": "CSV file uploaded and parsed successfully",
        "data": usage_data,
        "summary": summary,
        "record_count": usage_data.len()
    })))
}

pub async fn append_csv(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut csv_content = String::new();
    let mut file_received = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "MULTIPART_ERROR",
            &format!("Error processing multipart data: {}", e),
        )
    })? {
        let field_name = field.name().unwrap_or("");
        
        if field_name == "csvFile" {
            let file_name = field.file_name().unwrap_or("unknown").to_string();
            
            // Validate file extension
            if !file_name.to_lowercase().ends_with(".csv") {
                return Err(create_error_response(
                    StatusCode::BAD_REQUEST,
                    "INVALID_FILE_TYPE",
                    "Only CSV files are allowed",
                ));
            }

            let data = field.bytes().await.map_err(|e| {
                create_error_response(
                    StatusCode::BAD_REQUEST,
                    "FILE_READ_ERROR",
                    &format!("Error reading file: {}", e),
                )
            })?;

            // Check file size
            if data.len() > MAX_FILE_SIZE {
                return Err(create_error_response(
                    StatusCode::PAYLOAD_TOO_LARGE,
                    "FILE_TOO_LARGE",
                    &format!("File size ({} bytes) exceeds maximum allowed size ({} bytes)", data.len(), MAX_FILE_SIZE),
                ));
            }

            csv_content = String::from_utf8(data.to_vec()).map_err(|_| {
                create_error_response(
                    StatusCode::BAD_REQUEST,
                    "INVALID_ENCODING",
                    "File must be UTF-8 encoded",
                )
            })?;

            file_received = true;
            break;
        }
    }

    if !file_received {
        return Err(create_error_response(
            StatusCode::BAD_REQUEST,
            "NO_FILE",
            "No CSV file provided. Please upload a file with field name 'csvFile'",
        ));
    }

    // Parse new CSV content
    let parser = CsvParser::new();
    let new_usage_data = parser.parse_csv(&csv_content).map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "CSV_PARSE_ERROR",
            &e,
        )
    })?;

    // Merge with existing data
    let combined_data = {
        let mut data_store = UPLOADED_DATA.lock().unwrap();
        data_store.extend(new_usage_data.clone());
        
        // Sort by date to maintain chronological order
        data_store.sort_by(|a, b| a.date.cmp(&b.date));
        
        data_store.clone()
    };

    // Calculate summary for combined data
    let summary = calculate_summary(&combined_data);

    Ok(Json(json!({
        "success": true,
        "message": "CSV file appended successfully",
        "data": combined_data,
        "summary": summary,
        "new_records": new_usage_data.len(),
        "total_records": combined_data.len()
    })))
}

fn create_error_response(
    status: StatusCode,
    code: &str,
    message: &str,
) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            success: false,
            error: ErrorDetails {
                code: code.to_string(),
                message: message.to_string(),
                details: None,
            },
        }),
    )
}

fn calculate_summary(usage_data: &[UsageData]) -> UsageSummary {
    if usage_data.is_empty() {
        return UsageSummary {
            total_cost: 0.0,
            total_tokens: 0,
            average_cost_per_day: 0.0,
            most_used_model: "N/A".to_string(),
            date_range: DateRange {
                start: "N/A".to_string(),
                end: "N/A".to_string(),
            },
            model_breakdown: vec![],
        };
    }

    let total_cost: f64 = usage_data.iter().map(|d| d.cost).sum();
    let total_tokens: u32 = usage_data.iter().map(|d| d.total_tokens).sum();

    // Calculate model breakdown
    let mut model_stats: HashMap<String, ModelStats> = HashMap::new();
    
    for data in usage_data {
        let stats = model_stats.entry(data.model.clone()).or_insert(ModelStats {
            model: data.model.clone(),
            total_requests: 0,
            total_tokens: 0,
            total_cost: 0.0,
            average_tokens_per_request: 0.0,
            cache_efficiency: 0.0,
        });
        
        stats.total_requests += 1;
        stats.total_tokens += data.total_tokens;
        stats.total_cost += data.cost;
    }

    // Calculate averages and cache efficiency
    for stats in model_stats.values_mut() {
        stats.average_tokens_per_request = stats.total_tokens as f64 / stats.total_requests as f64;
        
        // Calculate cache efficiency for this model
        let model_data: Vec<_> = usage_data.iter().filter(|d| d.model == stats.model).collect();
        let total_cache_read: u32 = model_data.iter().map(|d| d.cache_read).sum();
        let total_input: u32 = model_data.iter().map(|d| d.input_with_cache + d.input_without_cache).sum();
        
        if total_input > 0 {
            stats.cache_efficiency = (total_cache_read as f64 / (total_cache_read + total_input) as f64) * 100.0;
        }
    }

    let model_breakdown: Vec<ModelStats> = model_stats.into_values().collect();
    
    // Find most used model by total requests
    let most_used_model = model_breakdown
        .iter()
        .max_by_key(|stats| stats.total_requests)
        .map(|stats| stats.model.clone())
        .unwrap_or_else(|| "N/A".to_string());

    // Calculate date range
    let dates: Vec<&String> = usage_data.iter().map(|d| &d.date).collect();
    let start_date = dates.iter().min().unwrap_or(&&"N/A".to_string()).to_string();
    let end_date = dates.iter().max().unwrap_or(&&"N/A".to_string()).to_string();

    // Calculate unique days for average cost per day
    let unique_days: std::collections::HashSet<String> = usage_data
        .iter()
        .map(|d| d.date.split('T').next().unwrap_or(&d.date).to_string())
        .collect();
    
    let average_cost_per_day = if unique_days.len() > 0 {
        total_cost / unique_days.len() as f64
    } else {
        0.0
    };

    UsageSummary {
        total_cost,
        total_tokens,
        average_cost_per_day,
        most_used_model,
        date_range: DateRange {
            start: start_date,
            end: end_date,
        },
        model_breakdown,
    }
}