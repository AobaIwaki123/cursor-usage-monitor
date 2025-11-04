use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

use crate::services::{data_processor::DataProcessor, stats_calculator::StatsCalculator};
use crate::storage::UPLOADED_DATA;

/// Comprehensive statistics endpoint
/// Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6
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

    // Calculate basic summary and model stats
    let processor = DataProcessor::new();
    let summary = processor.calculate_summary(&data);
    let model_stats = processor.calculate_model_stats(&data);

    // Calculate comprehensive statistics
    let stats_calculator = StatsCalculator::new();
    let comprehensive_stats = stats_calculator.calculate_comprehensive_stats(&data);

    Ok(Json(json!({
        "success": true,
        "summary": summary,
        "model_stats": model_stats,
        "comprehensive_stats": comprehensive_stats,
        "record_count": data.len(),
        "message": "Comprehensive statistics calculated successfully."
    })))
}