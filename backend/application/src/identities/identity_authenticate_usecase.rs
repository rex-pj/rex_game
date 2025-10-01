use std::sync::Arc;

use super::{
    identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    login_claims::LoginClaims,
};
use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    users::user_usecase_trait::UserUseCaseTrait,
};
use chrono::{Duration, Utc};
use rex_game_domain::{
    helpers::configuration_helper_trait::ConfigurationHelperTrait,
    identities::{
        password_hasher_trait::PasswordHasherTrait, token_helper_trait::TokenHelperTrait,
        TokenGenerationOptions, TokenValidationResult,
    },
};
use rex_game_shared::enums::user_token_porposes::UserTokenPurposes;

#[derive(Clone)]
pub struct IdentityAuthenticateUseCase<CF, PH, US, TH>
where
    CF: ConfigurationHelperTrait,
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    _password_hasher: PH,
    _user_usecase: US,
    _token_helper: TH,
    _configuration_helper: Arc<CF>,
}

impl<CF, PH, US, TH> IdentityAuthenticateUseCase<CF, PH, US, TH>
where
    CF: ConfigurationHelperTrait,
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    pub fn new(
        configuration_helper: Arc<CF>,
        password_hasher: PH,
        user_usecase: US,
        token_helper: TH,
    ) -> Self {
        Self {
            _configuration_helper: configuration_helper,
            _password_hasher: password_hasher,
            _user_usecase: user_usecase,
            _token_helper: token_helper,
        }
    }
}

impl<CF, PH, US, TH> IdentityAuthenticateUseCaseTrait
    for IdentityAuthenticateUseCase<CF, PH, US, TH>
where
    CF: ConfigurationHelperTrait,
    PH: PasswordHasherTrait,
    US: UserUseCaseTrait,
    TH: TokenHelperTrait,
{
    async fn password_login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        let existing_user = match self._user_usecase.get_user_by_email(email).await {
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

        let expiration = self
            ._configuration_helper
            .get_value::<i64>("identity.expiration");
        let generated_access_token_options = TokenGenerationOptions {
            email: Some(email.to_string()),
            user_id: existing_user.id,
            exp_secs: Duration::milliseconds(expiration).num_seconds(),
            purpose: UserTokenPurposes::Login.to_string(),
            iat: Some(Utc::now().timestamp()),
        };

        let generated_access_token = match self
            ._token_helper
            .generate_token(generated_access_token_options)
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

        let refresh_expiration = self
            ._configuration_helper
            .get_value::<i64>("identity.refresh_expiration");
        let generated_refresh_token_options = TokenGenerationOptions {
            email: None,
            user_id: existing_user.id,
            exp_secs: Duration::milliseconds(refresh_expiration).num_seconds(),
            purpose: UserTokenPurposes::RefreshToken.to_string(),
            iat: Some(Utc::now().timestamp()),
        };
        let generated_refresh_token = match self
            ._token_helper
            .generate_token(generated_refresh_token_options)
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
            access_token: generated_access_token.token,
            refresh_token: generated_refresh_token.token,
            refresh_token_expiration: generated_refresh_token.exp,
            email: email.to_string(),
            sub: generated_access_token.sub,
            expiration: generated_access_token.exp,
        })
    }

    async fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Result<LoginClaims, ApplicationError> {
        let refresh_expiration = self
            ._configuration_helper
            .get_value::<i64>("identity.refresh_expiration");
        let access_token_claims = match self._token_helper.refresh_access_token(
            access_token,
            refresh_token,
            refresh_expiration,
        ) {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError {
                    kind: ApplicationErrorKind::InvalidInput,
                    message: String::from("Failed to refresh access token"),
                    details: None,
                })
            }
        };

        let email = access_token_claims.email.ok_or_else(|| {
            ApplicationError::new(ApplicationErrorKind::NotFound, "No email found", None)
        })?;

        let refresh_expiration = self
            ._configuration_helper
            .get_value::<i64>("identity.refresh_expiration");
        let generated_token_options = TokenGenerationOptions {
            email: Some(email.to_owned()),
            user_id: access_token_claims.sub,
            exp_secs: Duration::milliseconds(refresh_expiration).num_seconds(),
            purpose: UserTokenPurposes::RefreshToken.to_string(),
            iat: Some(Utc::now().timestamp()),
        };
        let generated_token = match self._token_helper.generate_token(generated_token_options) {
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
            access_token: access_token_claims.token,
            refresh_token: generated_token.token,
            refresh_token_expiration: generated_token.exp,
            email: email,
            sub: access_token_claims.sub,
            expiration: access_token_claims.exp,
        })
    }

    fn validate_token(
        &self,
        access_token: &str,
    ) -> Result<TokenValidationResult, ApplicationError> {
        let verify_result = self._token_helper.validate_token(access_token);

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
