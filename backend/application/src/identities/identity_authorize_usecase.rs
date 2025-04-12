use std::{collections::HashSet, future::Future, pin::Pin};

use super::identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait;
use crate::errors::application_error::{ApplicationError, ErrorKind};
use rex_game_domain::repositories::user_role_repository_trait::UserRoleRepositoryTrait;

#[derive(Clone)]
pub struct IdentityAuthorizeUseCase<UR>
where
    UR: UserRoleRepositoryTrait,
{
    _user_role_repository: UR,
}

impl<UR: UserRoleRepositoryTrait> IdentityAuthorizeUseCase<UR> {
    pub fn new(user_role_repository: UR) -> Self {
        Self {
            _user_role_repository: user_role_repository,
        }
    }
}

impl<UR: UserRoleRepositoryTrait> IdentityAuthorizeUseCaseTrait for IdentityAuthorizeUseCase<UR>
where
    UR: UserRoleRepositoryTrait + Send + Sync + Clone + 'static,
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
                .map_err(|_| ApplicationError {
                    kind: ErrorKind::Unauthorized,
                    message: String::from("Unauthorized"),
                    details: None,
                })?;

            Ok(is_in_role)
        })
    }
}
