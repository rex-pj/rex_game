use rex_game_shared::ApplicationError;

use crate::domain::services::TokenValidationResult;

use super::login_claims::LoginClaims;
use std::future::Future;

pub trait IdentityAuthenticateUseCaseTrait {
    fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<LoginClaims, ApplicationError>>;
    fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> impl Future<Output = Result<LoginClaims, ApplicationError>>;

    fn validate_token(&self, access_token: &str)
        -> Result<TokenValidationResult, ApplicationError>;
}
