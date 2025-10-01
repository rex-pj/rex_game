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

pub struct TokenValidationResult {
    pub sub: i32,
    pub email: Option<String>,
    pub exp: u64,
    pub iss: String,
    pub token_type: String,
    pub iat: Option<i64>,
    pub jti: String,
}

pub struct TokenGenerationResult {
    pub sub: i32,
    pub token: String,
    pub email: Option<String>,
    pub exp: u64,
    pub token_type: String,
}

pub struct AccessTokenResult {
    pub sub: i32,
    pub aud: String,
    pub email: Option<String>,
    pub iss: String,
    pub exp: u64,
    pub token_type: String,
}

pub struct TokenGenerationOptions {
    pub user_id: i32,
    pub email: Option<String>,
    pub exp_secs: i64,
    pub purpose: String,
    pub iat: Option<i64>,
}
