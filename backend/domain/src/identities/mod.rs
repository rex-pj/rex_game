pub mod password_hasher_trait;
pub mod token_helper_trait;

pub struct IdentityError {
    pub kind: IdentityErrorKind,
    pub message: String,
    pub details: Option<String>,
}

pub enum IdentityErrorKind {
    NotFound,
    InvalidInput,
    DatabaseError,
}
