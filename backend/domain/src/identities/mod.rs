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
    Unauthorized,
}

pub struct IdentityClaims {
    pub sub: String,
    pub exp: u64,
    pub iss: String,
    pub token_type: String,
}

pub struct UserAccessClaims {
    pub access_token: String,
    // pub refresh_token: String,
    pub email: String,
    pub name: String,
    pub expiration: u64,
}
