use crate::helpers::configuration_helper::ConfigurationHelper;

use super::{IdentityRefreshTokenClaims, IdentityTokenClaims};
use jsonwebtoken::{encode, EncodingKey, Header};
use rex_game_domain::identities::token_helper_trait::TokenHelperTrait;

impl IdentityTokenHelper {
    pub fn new() -> Self {
        Self
    }
}

impl TokenHelperTrait for IdentityTokenHelper {
    fn generate_token(&self, user_name: &str, email: &str) -> Option<String> {
        let client_id = ConfigurationHelper::get_value("identity.issuer");
        let app_name = ConfigurationHelper::get_value("identity.app_name");
        let client_secret = ConfigurationHelper::get_value("identity.secret");
        let expiration: u64 = ConfigurationHelper::get_value("identity.expiration")
            .parse()
            .unwrap();

        let claims = IdentityTokenClaims {
            aud: user_name.to_owned(),
            sub: email.to_owned(),
            iss: client_id,
            company: app_name,
            exp: expiration,
            token_type: String::from("access"),
        };

        let token_result = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(client_secret.as_bytes()),
        );

        match token_result {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }

    fn generate_refresh_token(&self, email: &str) -> Option<String> {
        let client_id = ConfigurationHelper::get_value("identity.issuer");
        let client_secret = ConfigurationHelper::get_value("identity.secret");
        let refresh_expiration: u64 = ConfigurationHelper::get_value("identity.refresh_expiration")
            .parse()
            .unwrap();
        let claims = IdentityRefreshTokenClaims {
            sub: email.to_owned(),
            token_type: String::from("refresh"),
            iss: client_id,
            exp: refresh_expiration,
        };

        let token_result = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(client_secret.as_bytes()),
        );

        match token_result {
            Ok(token) => Some(token),
            Err(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct IdentityTokenHelper;
