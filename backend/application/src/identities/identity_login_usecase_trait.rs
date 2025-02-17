use super::login_claims::LoginClaims;
use crate::errors::application_error::ApplicationError;
use std::future::Future;

pub trait IdentityLoginUseCaseTrait {
    fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> impl Future<Output = Result<LoginClaims, ApplicationError>>;
}
