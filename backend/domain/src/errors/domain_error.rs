pub struct DomainError {
    pub kind: ErrorType,
    pub message: String,
    pub details: Option<String>,
}

impl DomainError {
    pub fn new(kind: ErrorType, message: &str, details: Option<String>) -> Self {
        Self {
            kind,
            message: String::from(message),
            details,
        }
    }
}

pub enum ErrorType {
    DatabaseError,
    NotFound,
}
