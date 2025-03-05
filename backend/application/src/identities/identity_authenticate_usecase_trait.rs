use super::login_claims::LoginClaims;
use crate::errors::application_error::ApplicationError;
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

    fn verify_access_token(
        &self,
        access_token: &str,
    ) -> impl Future<Output = Result<bool, ApplicationError>>;
}
