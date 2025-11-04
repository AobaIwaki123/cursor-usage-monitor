use std::sync::{Arc, Mutex};
use crate::models::usage_data::UsageData;

// Shared in-memory storage for uploaded data
lazy_static::lazy_static! {
    pub static ref UPLOADED_DATA: Arc<Mutex<Vec<UsageData>>> = Arc::new(Mutex::new(Vec::new()));
}