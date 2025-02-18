use rex_game_domain::identities::{
    password_hasher_trait::PasswordHasherTrait, token_helper_trait::TokenHelperTrait,
};

use super::{identity_login_usecase_trait::IdentityLoginUseCaseTrait, login_claims::LoginClaims};
use crate::{
    errors::application_error::{ApplicationError, ErrorKind},
    users::user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct IdentityLoginUseCase<PH, US, TH>
where
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
    _token_helper: TH,
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait, TH: TokenHelperTrait>
    IdentityLoginUseCase<PH, US, TH>
{
    pub fn new(password_hasher: PH, user_usecase: US, token_helper: TH) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
            _token_helper: token_helper,
        }
    }
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait, TH: TokenHelperTrait> IdentityLoginUseCaseTrait
    for IdentityLoginUseCase<PH, US, TH>
{
    async fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        let login_result = self
            ._user_usecase
            .get_user_by_email(String::from(email))
            .await;
        match login_result {
            Ok(existing_user) => {
                let password_hash = self
                    ._password_hasher
                    .hash(password, 16, existing_user.security_stamp)
                    .unwrap();
                if existing_user.password_hash == password_hash {
                    let token = self
                        ._token_helper
                        .generate_token(&existing_user.name, email)
                        .unwrap();

                    let refresh_token = self._token_helper.generate_refresh_token(email).unwrap();
                    return Ok(LoginClaims {
                        display_name: existing_user.display_name,
                        token: token,
                        user_email: existing_user.email,
                        user_id: existing_user.id,
                        refresh_token: refresh_token,
                    });
                }

                Err(ApplicationError {
                    kind: ErrorKind::InvalidInput,
                    message: String::from("Wrong information"),
                    details: None,
                })
            }
            Err(_) => Err(ApplicationError {
                kind: ErrorKind::InvalidInput,
                message: String::from("Login failed"),
                details: None,
            }),
        }
    }
}
