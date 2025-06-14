use crate::identities::AccessTokenInfo;

use super::{IdentityClaims, IdentityError, UserAccessClaims, UserRefreshTokenClaims};

pub trait TokenHelperTrait {
    fn generate_access_token(&self, user_id: i32, email: &str) -> Option<UserAccessClaims>;
    fn generate_refresh_token(&self, id: i32, email: &str) -> Option<UserRefreshTokenClaims>;
    fn refresh_access_token(&self, token: &str, refresh_token: &str) -> Option<UserAccessClaims>;
    fn validate_access_token(&self, access_token: &str) -> Result<IdentityClaims, IdentityError>;
    fn get_access_token_info(&self, access_token: &str) -> Option<AccessTokenInfo>;
}
