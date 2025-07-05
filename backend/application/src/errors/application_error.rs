pub struct ApplicationError {
    pub kind: ApplicationErrorKind,
    pub message: String,
    pub details: Option<String>,
}

impl ApplicationError {
    pub fn new(kind: ApplicationErrorKind, message: &str, details: Option<String>) -> Self {
        Self {
            kind,
            message: String::from(message),
            details,
        }
    }
}

pub enum ApplicationErrorKind {
    NotFound,
    InvalidInput,
    DatabaseError,
    Unauthorized,
}
