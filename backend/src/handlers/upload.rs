use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

// Placeholder for upload endpoints - will be implemented in task 2.2
pub async fn upload_csv() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Upload endpoint - to be implemented"
    })))
}

pub async fn append_csv() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Append endpoint - to be implemented"
    })))
}