use std::sync::Arc;

use super::{IdentityAccessTokenClaims, IdentityRefreshTokenClaims};
use chrono::{Duration, Utc};
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    Header, Validation,
};
use rex_game_domain::identities::AccessTokenInfo;
use rex_game_domain::identities::IdentityErrorKind;
use rex_game_domain::identities::UserRefreshTokenClaims;
use rex_game_domain::{
    helpers::configuration_helper_trait::ConfigurationHelperTrait,
    identities::{
        token_helper_trait::TokenHelperTrait, IdentityClaims, IdentityError, UserAccessClaims,
    },
};

#[derive(Clone)]
pub struct IdentityTokenHelper<CF: ConfigurationHelperTrait> {
    _configuration_helper: Arc<CF>,
    _client_id: Arc<String>,
    _client_secret: String,
    _app_name: Arc<String>,
    _expiration: i64,
    _refresh_expiration: i64,
}

impl<CF: ConfigurationHelperTrait> IdentityTokenHelper<CF> {
    pub fn new(configuration_helper: Arc<CF>) -> Self {
        return Self {
            _client_id: Arc::new(configuration_helper.get_value("identity.client_id")),
            _client_secret: configuration_helper.get_value("identity.client_secret"),
            _app_name: Arc::new(configuration_helper.get_value("identity.app_name")),
            _expiration: configuration_helper
                .get_value("identity.expiration")
                .parse()
                .unwrap(),
            _refresh_expiration: configuration_helper
                .get_value("identity.refresh_expiration")
                .parse()
                .unwrap(),
            _configuration_helper: configuration_helper,
        };
    }

    fn get_access_token_claims(
        &self,
        access_token: &str,
    ) -> Result<IdentityAccessTokenClaims, Error> {
        if access_token.is_empty() {
            return Err(Error::from(ErrorKind::InvalidToken));
        }
        let mut validation: Validation = Validation::default();
        validation.set_audience(&[self._client_id.to_string()]);
        validation.set_issuer(&[self._client_id.to_string()]);
        match decode::<IdentityAccessTokenClaims>(
            access_token,
            &DecodingKey::from_secret(self._client_secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => return Err(Error::from(ErrorKind::InvalidToken)),
        }
    }

    fn get_refresh_token_claims(
        &self,
        refresh_token: &str,
    ) -> Result<IdentityRefreshTokenClaims, Error> {
        if refresh_token.is_empty() {
            return Err(Error::from(ErrorKind::InvalidToken));
        }
        let mut validation: Validation = Validation::default();
        validation.set_audience(&[self._client_id.to_string()]);
        validation.set_issuer(&[self._client_id.to_string()]);
        let token_claims = match decode::<IdentityRefreshTokenClaims>(
            refresh_token,
            &DecodingKey::from_secret(self._client_secret.as_bytes()),
            &validation,
        ) {
            Ok(token_data) => token_data.claims,
            Err(_) => return Err(Error::from(ErrorKind::InvalidToken)),
        };

        let now = Utc::now().timestamp() as u64;
        if token_claims.exp < now {
            return Err(Error::from(ErrorKind::InvalidToken));
        }

        Ok(token_claims)
    }
}

impl<CF: ConfigurationHelperTrait> TokenHelperTrait for IdentityTokenHelper<CF> {
    fn get_access_token_info(&self, access_token: &str) -> Option<AccessTokenInfo> {
        if access_token.is_empty() {
            return None;
        }
        let access_claims = self.get_access_token_claims(access_token);
        match access_claims {
            Ok(claims) => Some(AccessTokenInfo {
                sub: claims.sub,
                aud: claims.aud,
                email: claims.email,
                company: claims.company,
                iss: claims.iss,
                exp: claims.exp,
                token_type: claims.token_type,
            }),
            Err(_) => None,
        }
    }

    fn generate_access_token(&self, user_id: i32, email: &str) -> Option<UserAccessClaims> {
        if user_id == 0 || email.is_empty() {
            return None;
        }
        let now = Utc::now();
        let claims = IdentityAccessTokenClaims {
            sub: user_id,
            aud: self._client_id.to_string(),
            email: email.to_owned(),
            iss: self._client_id.to_string(),
            company: self._app_name.to_string(),
            exp: (now + Duration::milliseconds(self._expiration)).timestamp() as u64,
            token_type: String::from("access"),
        };

        let token_result = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self._client_secret.as_bytes()),
        );

        match token_result {
            Ok(token) => Some(UserAccessClaims {
                sub: user_id,
                access_token: token,
                email: email.to_owned(),
                expiration: claims.exp,
            }),
            Err(_) => None,
        }
    }

    fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> Option<UserAccessClaims> {
        if access_token.is_empty() || refresh_token.is_empty() {
            return None;
        }
        let access_claims = self.get_access_token_claims(access_token).unwrap();
        let refresh_claims = self.get_refresh_token_claims(refresh_token);

        match refresh_claims {
            Ok(rf_token_claims) => {
                if access_claims.sub != rf_token_claims.sub {
                    return None;
                }

                self.generate_access_token(access_claims.sub, &access_claims.email)
            }
            Err(_) => None,
        }
    }

    fn generate_refresh_token(&self, id: i32, email: &str) -> Option<UserRefreshTokenClaims> {
        if id == 0 || email.is_empty() {
            return None;
        }
        let now = Utc::now();
        let claims = IdentityRefreshTokenClaims {
            sub: id,
            email: email.to_owned(),
            token_type: String::from("refresh"),
            iss: self._client_id.to_string(),
            aud: self._client_id.to_string(),
            exp: (now + Duration::milliseconds(self._refresh_expiration)).timestamp() as u64,
        };

        let token_result = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self._client_secret.as_bytes()),
        );

        match token_result {
            Ok(token) => Some(UserRefreshTokenClaims {
                refresh_token: token,
                expiration: claims.exp,
            }),
            Err(_) => None,
        }
    }

    fn validate_access_token(&self, access_token: &str) -> Result<IdentityClaims, IdentityError> {
        if access_token.is_empty() {
            return Err(IdentityError {
                kind: IdentityErrorKind::InvalidInput,
                message: String::from("No token"),
                details: None,
            });
        }

        let token_data_claims = match self.get_access_token_claims(access_token) {
            Ok(claims) => claims,
            Err(_) => {
                return Err(IdentityError {
                    kind: IdentityErrorKind::Unauthorized,
                    message: String::from("Token is invalid"),
                    details: None,
                })
            }
        };

        let now = Utc::now().timestamp() as u64;
        if token_data_claims.exp < now {
            return Err(IdentityError {
                kind: IdentityErrorKind::Unauthorized,
                message: String::from("Token is invalid or expired"),
                details: None,
            });
        }

        Ok(IdentityClaims {
            exp: token_data_claims.exp,
            iss: token_data_claims.iss,
            sub: token_data_claims.sub,
            email: token_data_claims.email,
            token_type: token_data_claims.token_type,
        })
    }
}
