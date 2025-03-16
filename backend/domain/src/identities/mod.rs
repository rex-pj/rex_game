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
    InternalServerError,
    DatabaseError,
    Unauthorized,
}

pub struct IdentityClaims {
    pub sub: i32,
    pub email: String,
    pub exp: u64,
    pub iss: String,
    pub token_type: String,
}

pub struct UserAccessClaims {
    pub sub: i32,
    pub access_token: String,
    pub email: String,
    pub expiration: u64,
}
