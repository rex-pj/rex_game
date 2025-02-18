pub trait TokenHelperTrait {
    fn generate_token(&self, user_name: &str, email: &str) -> Option<String>;
    fn generate_refresh_token(&self, email: &str) -> Option<String>;
}
