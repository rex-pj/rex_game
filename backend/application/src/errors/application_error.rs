pub struct ApplicationError {
    pub kind: ErrorKind,
    pub message: String,
    pub details: Option<String>,
}

pub enum ErrorKind {
    NotFound,
    InvalidInput,
    DatabaseError,
    Unauthorized,
}
