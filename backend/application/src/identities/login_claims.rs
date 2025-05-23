pub struct LoginClaims {
    pub access_token: String,
    pub refresh_token: String,
    pub refresh_token_expiration: u64,
    pub email: String,
    pub sub: i32,
    pub expiration: u64,
}
