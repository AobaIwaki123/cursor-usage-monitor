// Validation utilities
#[allow(dead_code)]
pub fn validate_file_size(size: u64) -> Result<(), String> {
    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
    
    if size > MAX_FILE_SIZE {
        return Err(format!("File size {} bytes exceeds maximum allowed size of {} bytes", size, MAX_FILE_SIZE));
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn validate_csv_extension(filename: &str) -> Result<(), String> {
    if !filename.to_lowercase().ends_with(".csv") {
        return Err("File must have .csv extension".to_string());
    }
    
    Ok(())
}