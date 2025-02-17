use rex_game_domain::identities::password_hasher_trait::PasswordHasherTrait;

use super::{identity_login_usecase_trait::IdentityLoginUseCaseTrait, login_claims::LoginClaims};
use crate::{
    errors::application_error::ApplicationError, users::user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct IdentityLoginUseCase<PH, US>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait> IdentityLoginUseCase<PH, US> {
    pub fn new(password_hasher: PH, user_usecase: US) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
        }
    }
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait> IdentityLoginUseCaseTrait
    for IdentityLoginUseCase<PH, US>
{
    async fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        todo!()
    }
}
