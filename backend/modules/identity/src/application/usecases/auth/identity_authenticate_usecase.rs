use std::sync::Arc;

use super::{
    identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    login_claims::LoginClaims,
};
use crate::application::usecases::user_usecase_trait::UserUseCaseTrait;
use crate::domain::services::{
    password_hasher_trait::PasswordHasherTrait,
    token_helper_trait::TokenHelperTrait,
    token_types::{TokenGenerationOptions, TokenValidationResult},
};
use chrono::{Duration, Utc};
use rex_game_shared::domain::configuration_helper_trait::ConfigurationHelperTrait;
use rex_game_shared::domain::enums::user_token_porposes::UserTokenPurposes;
use rex_game_shared::ApplicationError;

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
            Err(err) => return Err(ApplicationError::invalid_input(err.to_string())),
        };

        // Note: Permissions and roles will be empty on initial login
        // They will be populated by the authorization middleware on subsequent requests
        // This is a simplified implementation - for full optimization,
        // create a dedicated repository method that fetches user with roles and permissions in one query
        let permissions: Vec<String> = vec![];
        let roles: Vec<String> = vec![];

        let expiration = self
            ._configuration_helper
            .get_value::<i64>("JWT_EXPIRATION");
        let generated_access_token_options = TokenGenerationOptions {
            email: Some(email.to_string()),
            user_id: existing_user.id,
            exp_secs: Duration::milliseconds(expiration).num_seconds(),
            purpose: UserTokenPurposes::Login.to_string(),
            iat: Some(Utc::now().timestamp()),
            permissions,
            roles,
        };

        let generated_access_token = match self
            ._token_helper
            .generate_token(generated_access_token_options)
        {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError::invalid_input(String::from(
                    "Failed to generate refresh token",
                )))
            }
        };

        let refresh_expiration = self
            ._configuration_helper
            .get_value::<i64>("JWT_REFRESH_EXPIRATION");
        let generated_refresh_token_options = TokenGenerationOptions {
            email: None,
            user_id: existing_user.id,
            exp_secs: Duration::milliseconds(refresh_expiration).num_seconds(),
            purpose: UserTokenPurposes::RefreshToken.to_string(),
            iat: Some(Utc::now().timestamp()),
            permissions: vec![], // Refresh tokens don't need permissions
            roles: vec![],
        };
        let generated_refresh_token = match self
            ._token_helper
            .generate_token(generated_refresh_token_options)
        {
            Some(refresh_token) => refresh_token,
            None => {
                return Err(ApplicationError::invalid_input(String::from(
                    "Failed to generate refresh token",
                )))
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
            .get_value::<i64>("JWT_REFRESH_EXPIRATION");
        let access_token_claims = match self._token_helper.refresh_access_token(
            access_token,
            refresh_token,
            refresh_expiration,
        ) {
            Some(claims) => claims,
            None => {
                return Err(ApplicationError::invalid_input(String::from(
                    "Failed to refresh access token",
                )))
            }
        };

        let email = access_token_claims
            .email
            .ok_or_else(|| ApplicationError::not_found("No email found", "".to_string()))?;

        let refresh_expiration = self
            ._configuration_helper
            .get_value::<i64>("JWT_REFRESH_EXPIRATION");
        let generated_token_options = TokenGenerationOptions {
            email: Some(email.to_owned()),
            user_id: access_token_claims.sub,
            exp_secs: Duration::milliseconds(refresh_expiration).num_seconds(),
            purpose: UserTokenPurposes::RefreshToken.to_string(),
            iat: Some(Utc::now().timestamp()),
            permissions: vec![],
            roles: vec![],
        };
        let generated_token = match self._token_helper.generate_token(generated_token_options) {
            Some(refresh_token) => refresh_token,
            None => {
                return Err(ApplicationError::invalid_input(String::from(
                    "Failed to generate refresh token",
                )))
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
            Err(_) => Err(ApplicationError::invalid_token("Invalid token")),
        }
    }
}
