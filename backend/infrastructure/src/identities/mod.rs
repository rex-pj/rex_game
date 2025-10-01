pub mod identity_password_hasher;
pub mod identity_token_helper;
use serde::{Deserialize, Serialize};

pub trait HasExpiryTokenClaimTrait {
    fn exp(&self) -> u64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: i32,
    pub aud: String,
    pub email: Option<String>,
    pub iss: String,
    pub exp: u64,
    pub token_type: String,
    pub iat: Option<i64>,
    pub jti: String,
}

impl HasExpiryTokenClaimTrait for AccessTokenClaims {
    fn exp(&self) -> u64 {
        self.exp
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: i32,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
    pub token_type: String,
    pub iat: i64,
    pub jti: String,
}

impl HasExpiryTokenClaimTrait for RefreshTokenClaims {
    fn exp(&self) -> u64 {
        self.exp
    }
}
