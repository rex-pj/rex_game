use super::{AccessTokenClaims, RefreshTokenClaims};
use crate::identities::HasExpiryTokenClaimTrait;
use chrono::{Duration, Utc};
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    Header, Validation,
};
use rex_game_domain::identities::IdentityErrorKind;
use rex_game_domain::identities::TokenGenerationOptions;
use rex_game_domain::identities::TokenGenerationResult;
use rex_game_domain::{
    helpers::configuration_helper_trait::ConfigurationHelperTrait,
    identities::{token_helper_trait::TokenHelperTrait, IdentityError, TokenValidationResult},
};
use rex_game_shared::enums::user_token_porposes::UserTokenPurposes;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct IdentityTokenHelper<CF: ConfigurationHelperTrait> {
    _configuration_helper: Arc<CF>,
    _client_id: Arc<String>,
    _client_secret: String,
}

impl<CF: ConfigurationHelperTrait> IdentityTokenHelper<CF> {
    pub fn new(configuration_helper: Arc<CF>) -> Self {
        return Self {
            _client_id: Arc::new(configuration_helper.get_value("identity.client_id")),
            _client_secret: configuration_helper.get_value("identity.client_secret"),
            _configuration_helper: configuration_helper,
        };
    }

    fn get_token_claims<T>(&self, access_token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned + HasExpiryTokenClaimTrait,
    {
        if access_token.is_empty() {
            return Err(Error::from(ErrorKind::InvalidToken));
        }
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[self._client_id.to_string()]);
        validation.set_issuer(&[self._client_id.to_string()]);
        let secret_decoding = DecodingKey::from_secret(self._client_secret.as_bytes());
        let token_claims = match decode::<T>(access_token, &secret_decoding, &validation) {
            Ok(token_data) => token_data.claims,
            Err(err) => return Err(err),
        };

        Ok(token_claims)
    }

    fn validate_token<T>(&self, access_token: &str) -> Result<T, Error>
    where
        T: DeserializeOwned + HasExpiryTokenClaimTrait,
    {
        if access_token.is_empty() {
            return Err(Error::from(ErrorKind::InvalidToken));
        }
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[self._client_id.to_string()]);
        validation.set_issuer(&[self._client_id.to_string()]);

        let secret_decoding = DecodingKey::from_secret(self._client_secret.as_bytes());
        let token_claims = match decode::<T>(access_token, &secret_decoding, &validation) {
            Ok(token_data) => token_data.claims,
            Err(err) => return Err(err),
        };

        Ok(token_claims)
    }
}

impl<CF: ConfigurationHelperTrait> TokenHelperTrait for IdentityTokenHelper<CF> {
    fn generate_token(&self, options: TokenGenerationOptions) -> Option<TokenGenerationResult> {
        if options.user_id == 0 {
            return None;
        }

        let now = Utc::now();
        let claims = AccessTokenClaims {
            sub: options.user_id,
            aud: self._client_id.to_string(),
            email: options.email.to_owned(),
            iss: self._client_id.to_string(),
            exp: (now + Duration::seconds(options.exp_secs)).timestamp() as u64,
            token_type: options.purpose.to_owned(),
            iat: options.iat,
            jti: Uuid::new_v4().to_string(),
        };

        let secret_encoding = EncodingKey::from_secret(self._client_secret.as_bytes());
        let token_result = encode(&Header::default(), &claims, &secret_encoding);

        match token_result {
            Ok(token) => Some(TokenGenerationResult {
                sub: options.user_id,
                token: token,
                email: options.email.to_owned(),
                exp: claims.exp,
                token_type: options.purpose,
            }),
            Err(_) => None,
        }
    }

    fn validate_token(&self, access_token: &str) -> Result<TokenValidationResult, IdentityError> {
        if access_token.is_empty() {
            return Err(IdentityError {
                kind: IdentityErrorKind::InvalidInput,
                message: String::from("No token"),
                details: None,
            });
        }

        let token_data_claims = match self.validate_token::<AccessTokenClaims>(access_token) {
            Ok(claims) => claims,
            Err(_) => {
                return Err(IdentityError {
                    kind: IdentityErrorKind::Unauthorized,
                    message: String::from("Token is invalid or expired"),
                    details: None,
                })
            }
        };

        Ok(TokenValidationResult {
            exp: token_data_claims.exp,
            iss: token_data_claims.iss,
            sub: token_data_claims.sub,
            email: token_data_claims.email,
            token_type: token_data_claims.token_type,
            iat: token_data_claims.iat,
            jti: token_data_claims.jti,
        })
    }

    fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
        refresh_expiration: i64,
    ) -> Option<TokenGenerationResult> {
        if access_token.is_empty() || refresh_token.is_empty() {
            return None;
        }

        let refresh_claims = self.get_token_claims::<RefreshTokenClaims>(refresh_token);

        match refresh_claims {
            Ok(rf_token_claims) => {
                let access_claims = self
                    .get_token_claims::<AccessTokenClaims>(access_token)
                    .unwrap();

                if access_claims.sub != rf_token_claims.sub {
                    return None;
                }

                let purpose = UserTokenPurposes::RefreshToken.to_string();
                let generated_token_options = TokenGenerationOptions {
                    email: access_claims.email,
                    user_id: access_claims.sub,
                    exp_secs: Duration::milliseconds(refresh_expiration).num_seconds(),
                    purpose: purpose,
                    iat: Some(Utc::now().timestamp()),
                };
                self.generate_token(generated_token_options)
            }
            Err(_) => None,
        }
    }
}
