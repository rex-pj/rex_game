pub mod identity_password_hasher;
pub mod identity_token_helper;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityAccessTokenClaims {
    pub aud: String,
    pub sub: String,
    pub company: String,
    pub iss: String,
    pub exp: u64,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityRefreshTokenClaims {
    pub aud: String,
    pub sub: String,
    pub exp: u64,
    pub iss: String,
    pub token_type: String,
}
