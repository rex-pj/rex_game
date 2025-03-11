pub mod identity_password_hasher;
pub mod identity_token_helper;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityAccessTokenClaims {
    pub sub: i32,
    pub aud: String,
    pub email: String,
    pub company: String,
    pub iss: String,
    pub exp: u64,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityRefreshTokenClaims {
    pub sub: i32,
    pub aud: String,
    pub email: String,
    pub exp: u64,
    pub iss: String,
    pub token_type: String,
}
