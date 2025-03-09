pub struct ApplicationError {
    pub kind: ErrorKind,
    pub message: String,
    pub details: Option<String>,
}

impl ApplicationError {
    pub fn new(kind: ErrorKind, message: &str, details: Option<String>) -> Self {
        Self {
            kind,
            message: String::from(message),
            details,
        }
    }
}

pub enum ErrorKind {
    NotFound,
    InvalidInput,
    DatabaseError,
    Unauthorized,
}
