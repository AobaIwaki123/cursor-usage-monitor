use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

use crate::services::data_processor::DataProcessor;
use crate::storage::UPLOADED_DATA;

// Basic stats endpoint using DataProcessor - comprehensive stats will be implemented in task 2.4
pub async fn comprehensive_stats() -> Result<Json<Value>, StatusCode> {
    let data = {
        let data_store = UPLOADED_DATA.lock().unwrap();
        data_store.clone()
    };

    if data.is_empty() {
        return Ok(Json(json!({
            "success": true,
            "message": "No data available. Please upload a CSV file first.",
            "data": null
        })));
    }

    let processor = DataProcessor::new();
    let summary = processor.calculate_summary(&data);
    let model_stats = processor.calculate_model_stats(&data);

    Ok(Json(json!({
        "success": true,
        "summary": summary,
        "model_stats": model_stats,
        "record_count": data.len(),
        "message": "Basic statistics calculated successfully. Comprehensive stats will be available in task 2.4."
    })))
}