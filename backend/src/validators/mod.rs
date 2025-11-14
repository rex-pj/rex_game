use validator::ValidationError;
pub mod validation_helper;

pub fn validate_content_type(content_type: &str) -> Result<(), ValidationError> {
    let allowed = ["image/jpeg", "image/png", "image/gif"];
    if allowed.contains(&content_type) {
        Ok(())
    } else {
        let err = ValidationError::new("invalid_content_type");
        Err(err)
    }
}

pub fn validate_file_size(data: &Vec<u8>) -> Result<(), ValidationError> {
    let max_size = 2 * 1024 * 1024;
    if data.len() <= max_size {
        Ok(())
    } else {
        Err(ValidationError::new("file_too_large"))
    }
}

pub fn validate_file_type(file_name: &str) -> Result<(), ValidationError> {
    let allowed_extensions = ["jpg", "jpeg", "png", "gif"];
    if let Some(ext) = file_name.split('.').last() {
        if allowed_extensions.contains(&ext.to_lowercase().as_str()) {
            return Ok(());
        }
    }
    Err(ValidationError::new("invalid_file_type"))
}

pub fn validate_file_content(data: &Vec<u8>) -> Result<(), ValidationError> {
    // Use infer to detect actual MIME type from file content (magic bytes)
    let mime = match infer::get(data) {
        Some(kind) => kind.mime_type(),
        None => "application/octet-stream", // Fallback if type cannot be detected
    };
    validate_content_type(mime)
}

/// Detects the actual content type from file data and returns it
/// Returns the detected MIME type if valid, otherwise returns an error
pub fn detect_content_type(data: &Vec<u8>) -> Result<String, ValidationError> {
    let mime = match infer::get(data) {
        Some(kind) => kind.mime_type(),
        None => {
            return Err(ValidationError::new("cannot_detect_content_type"));
        }
    };
    validate_content_type(mime)?;
    Ok(mime.to_string())
}
