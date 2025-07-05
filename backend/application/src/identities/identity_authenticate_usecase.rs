use rex_game_domain::identities::{
    password_hasher_trait::PasswordHasherTrait, token_helper_trait::TokenHelperTrait,
    IdentityClaims,
};

use super::{
    identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    login_claims::LoginClaims,
};
use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    users::user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct IdentityAuthenticateUseCase<PH, US, TH>
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
    IdentityAuthenticateUseCase<PH, US, TH>
{
    pub fn new(password_hasher: PH, user_usecase: US, token_helper: TH) -> Self {
        Self {
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
            _token_helper: token_helper,
        }
    }
}

impl<PH: PasswordHasherTrait, US: UserUseCaseTrait, TH: TokenHelperTrait>
    IdentityAuthenticateUseCaseTrait for IdentityAuthenticateUseCase<PH, US, TH>
{
    async fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        let existing_user = match self
            ._user_usecase
            .get_user_by_email(String::from(email))
            .await
        {
            Ok(existing_user) => existing_user,
            Err(err) => return Err(err),
        };

        match self
            ._password_hasher
            .verify_password(password, &existing_user.password_hash)
        {
            Ok(_) => true,
            Err(e) => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: e.message,
                    details: None,
                })
            }
        };

        let access_token_claims = match self
            ._token_helper
            .generate_access_token(existing_user.id, email)
        {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to generate refresh token"),
                    details: None,
                })
            }
        };

        let refresh_token_claims = match self
            ._token_helper
            .generate_refresh_token(existing_user.id, email)
        {
            Some(refresh_token) => refresh_token,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to generate refresh token"),
                    details: None,
                })
            }
        };

        Ok(LoginClaims {
            access_token: access_token_claims.access_token,
            refresh_token: refresh_token_claims.refresh_token,
            refresh_token_expiration: refresh_token_claims.expiration,
            email: email.to_string(),
            sub: access_token_claims.sub,
            expiration: access_token_claims.expiration,
        })
    }

    async fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        let access_token_claims = match self
            ._token_helper
            .refresh_access_token(access_token, refresh_token)
        {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to refresh access token"),
                    details: None,
                })
            }
        };

        let user_refresh_token_claims = match self
            ._token_helper
            .generate_refresh_token(access_token_claims.sub, &access_token_claims.email)
        {
            Some(refresh_token) => refresh_token,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to generate refresh token"),
                    details: None,
                })
            }
        };

        Ok(LoginClaims {
            access_token: access_token_claims.access_token,
            refresh_token: user_refresh_token_claims.refresh_token,
            refresh_token_expiration: user_refresh_token_claims.expiration,
            email: access_token_claims.email,
            sub: access_token_claims.sub,
            expiration: access_token_claims.expiration,
        })
    }

    fn verify_access_token(&self, access_token: &str) -> Result<IdentityClaims, ApplicationError> {
        let verify_result = self._token_helper.validate_access_token(access_token);

        match verify_result {
            Ok(claims) => Ok(claims),
            Err(_) => Err(ApplicationError {
                kind: ApplicationErrorKind::Unauthorized,
                message: String::from("Unauthorized"),
                details: None,
            }),
        }
    }
}
