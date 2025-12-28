use std::{collections::HashSet, future::Future, pin::Pin};

use rex_game_shared::ApplicationError;

use super::identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait;
use crate::domain::repositories::{
    role_permission_repository_trait::RolePermissionRepositoryTrait,
    user_permission_repository_trait::UserPermissionRepositoryTrait,
    user_role_repository_trait::UserRoleRepositoryTrait,
};

#[derive(Clone)]
pub struct IdentityAuthorizeUseCase<UR, UP, RP>
where
    UR: UserRoleRepositoryTrait,
    UP: UserPermissionRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
{
    _user_role_repository: UR,
    _user_permission_repository: UP,
    _role_permission_repository: RP,
}

impl<
        UR: UserRoleRepositoryTrait,
        UP: UserPermissionRepositoryTrait,
        RP: RolePermissionRepositoryTrait,
    > IdentityAuthorizeUseCase<UR, UP, RP>
{
    pub fn new(
        user_role_repository: UR,
        user_permission_repository: UP,
        role_permission_repository: RP,
    ) -> Self {
        Self {
            _user_role_repository: user_role_repository,
            _user_permission_repository: user_permission_repository,
            _role_permission_repository: role_permission_repository,
        }
    }
}

impl<
        UR: UserRoleRepositoryTrait,
        UP: UserPermissionRepositoryTrait,
        RP: RolePermissionRepositoryTrait,
    > IdentityAuthorizeUseCaseTrait for IdentityAuthorizeUseCase<UR, UP, RP>
where
    UR: UserRoleRepositoryTrait + Send + Sync + Clone + 'static,
    UP: UserPermissionRepositoryTrait + Send + Sync + Clone + 'static,
    RP: RolePermissionRepositoryTrait + Send + Sync + Clone + 'static,
{
    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>> {
        let user_role_repository = self._user_role_repository.clone();
        // Ownership of roles is now transferred, avoiding lifetime issues.
        Box::pin(async move {
            let is_in_role = user_role_repository
                .is_user_in_role(user_id, roles)
                .await
                .map_err(|_| ApplicationError::unauthorized("User not in role"))?;

            Ok(is_in_role)
        })
    }

    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>> {
        let user_permission_repository = self._user_permission_repository.clone();
        // Ownership of permissions is now transferred, avoiding lifetime issues.
        Box::pin(async move {
            let is_in_permission = user_permission_repository
                .is_user_in_permission(user_id, permission_codes)
                .await
                .map_err(|_| ApplicationError::unauthorized("User not in permission"))?;
            Ok(is_in_permission)
        })
    }

    fn are_roles_in_permission(
        &self,
        role_ids: Vec<i32>,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>> {
        let user_permission_repository = self._role_permission_repository.clone();
        // Ownership of permissions is now transferred, avoiding lifetime issues.
        Box::pin(async move {
            if role_ids.is_empty() {
                return Ok(false);
            }
            let is_in_permission = user_permission_repository
                .are_roles_in_permission(role_ids, permission_codes)
                .await
                .map_err(|_| ApplicationError::unauthorized("Roles not in permission"))?;
            Ok(is_in_permission)
        })
    }
}
