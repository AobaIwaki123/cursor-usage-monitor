use chrono::{DateTime, Utc, NaiveDateTime};

// Date utility functions
#[allow(dead_code)]
pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, String> {
    // Try parsing ISO 8601 format first
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Try parsing naive datetime and assume UTC
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc));
    }
    
    Err(format!("Unable to parse date: {}", date_str))
}

#[allow(dead_code)]
pub fn format_date_for_display(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}