pub struct LoginClaims {
    pub token: String,
    pub refresh_token: String,
    pub user_id: i32,
    pub user_email: String,
    pub display_name: Option<String>,
}
