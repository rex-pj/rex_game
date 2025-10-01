use tree_magic_mini::from_u8;
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
    let mime = from_u8(data);
    validate_content_type(mime)
}
