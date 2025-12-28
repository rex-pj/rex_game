pub struct IdentityError {
    pub kind: IdentityErrorKind,
    pub message: String,
    pub details: Option<String>,
}

pub enum IdentityErrorKind {
    NotFound,
    InvalidInput,
    InternalServerError,
    DatabaseError,
    Unauthorized,
}
