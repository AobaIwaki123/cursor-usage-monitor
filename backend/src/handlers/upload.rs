use axum::{
    extract::Multipart,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use crate::models::error::{ErrorResponse, ErrorDetails};
use crate::services::{csv_parser::CsvParser, data_processor::DataProcessor};
use crate::storage::UPLOADED_DATA;

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

    // Validate parsed data
    let processor = DataProcessor::new();
    processor.validate_usage_data(&usage_data).map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "DATA_VALIDATION_ERROR",
            &e,
        )
    })?;

    // Store data in memory (replace existing data)
    {
        let mut data_store = UPLOADED_DATA.lock().unwrap();
        *data_store = usage_data.clone();
    }

    // Calculate summary using DataProcessor
    let summary = processor.calculate_summary(&usage_data);

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

    // Validate new data
    let processor = DataProcessor::new();
    processor.validate_usage_data(&new_usage_data).map_err(|e| {
        create_error_response(
            StatusCode::BAD_REQUEST,
            "DATA_VALIDATION_ERROR",
            &e,
        )
    })?;

    // Merge with existing data using DataProcessor
    let combined_data = {
        let existing_data = {
            let data_store = UPLOADED_DATA.lock().unwrap();
            data_store.clone()
        };
        
        let merged_data = processor.merge_data(existing_data, new_usage_data.clone());
        
        // Update stored data
        {
            let mut data_store = UPLOADED_DATA.lock().unwrap();
            *data_store = merged_data.clone();
        }
        
        merged_data
    };

    // Calculate summary for combined data using DataProcessor
    let summary = processor.calculate_summary(&combined_data);

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

