use std::{collections::HashSet, future::Future, pin::Pin};

use rex_game_shared::ApplicationError;

pub trait IdentityAuthorizeUseCaseTrait {
    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>>;
    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>>;
    fn are_roles_in_permission(
        &self,
        role_ids: Vec<i32>,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>>;
}
