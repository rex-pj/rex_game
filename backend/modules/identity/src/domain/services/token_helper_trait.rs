use rex_game_shared::ApplicationError;

use super::token_types::{TokenGenerationOptions, TokenGenerationResult, TokenValidationResult};

pub trait TokenHelperTrait {
    fn generate_token(&self, options: TokenGenerationOptions) -> Option<TokenGenerationResult>;
    fn refresh_access_token(
        &self,
        token: &str,
        refresh_token: &str,
        refresh_expiration: i64,
    ) -> Option<TokenGenerationResult>;
    fn validate_token(&self, access_token: &str)
        -> Result<TokenValidationResult, ApplicationError>;
}
