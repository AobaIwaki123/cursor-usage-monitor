use axum::{extract::Query, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use chrono::{DateTime, NaiveDate};

use crate::models::usage_data::UsageData;
use crate::services::{data_processor::DataProcessor, stats_calculator::StatsCalculator};
use crate::storage::UPLOADED_DATA;

#[derive(Debug, Deserialize, Serialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Filter usage data by date range
/// Requirements: 10.1, 10.2, 10.5
fn filter_by_date_range(data: &[UsageData], query: &DateRangeQuery) -> Vec<UsageData> {
    if query.start_date.is_none() && query.end_date.is_none() {
        return data.to_vec();
    }

    data.iter()
        .filter(|usage| {
            // Parse the usage date
            let usage_date = match DateTime::parse_from_rfc3339(&usage.date) {
                Ok(dt) => dt.date_naive(),
                Err(_) => return false,
            };

            // Check start date
            if let Some(ref start) = query.start_date {
                if let Ok(start_date) = NaiveDate::parse_from_str(start, "%Y-%m-%d") {
                    if usage_date < start_date {
                        return false;
                    }
                }
            }

            // Check end date
            if let Some(ref end) = query.end_date {
                if let Ok(end_date) = NaiveDate::parse_from_str(end, "%Y-%m-%d") {
                    if usage_date > end_date {
                        return false;
                    }
                }
            }

            true
        })
        .cloned()
        .collect()
}

/// Comprehensive statistics endpoint with date filtering
/// Requirements: 7.1, 7.2, 7.3, 7.4, 7.5, 7.6, 10.1, 10.2, 10.5
pub async fn comprehensive_stats(
    Query(date_range): Query<DateRangeQuery>,
) -> Result<Json<Value>, StatusCode> {
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

    // Filter data by date range if provided
    let filtered_data = filter_by_date_range(&data, &date_range);

    if filtered_data.is_empty() {
        return Ok(Json(json!({
            "success": true,
            "message": "No data found for the specified date range.",
            "data": null,
            "date_range": date_range
        })));
    }

    // Calculate basic summary and model stats
    let processor = DataProcessor::new();
    let summary = processor.calculate_summary(&filtered_data);
    let model_stats = processor.calculate_model_stats(&filtered_data);

    // Calculate comprehensive statistics
    let stats_calculator = StatsCalculator::new();
    let comprehensive_stats = stats_calculator.calculate_comprehensive_stats(&filtered_data);

    Ok(Json(json!({
        "success": true,
        "summary": summary,
        "model_stats": model_stats,
        "comprehensive_stats": comprehensive_stats,
        "record_count": filtered_data.len(),
        "total_records": data.len(),
        "date_range": date_range,
        "message": "Comprehensive statistics calculated successfully."
    })))
}