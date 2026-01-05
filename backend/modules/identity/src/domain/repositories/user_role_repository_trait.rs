use crate::domain::models::user_role_model::UserRoleModel;
use rex_game_shared::domain::transaction_manager_trait::TransactionWrapperTrait;
use rex_game_shared::InfraError;
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait UserRoleRepositoryTrait {
    fn create_without_commit(
        &self,
        user_role: UserRoleModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, InfraError>>;
    fn create_many(
        &self,
        user_role_req: Vec<UserRoleModel>,
    ) -> impl Future<Output = Result<i32, InfraError>> + Send;
    fn delete_many(
        &self,
        user_id: i32,
        user_role_req: Vec<UserRoleModel>,
    ) -> impl Future<Output = Result<u64, InfraError>> + Send;
    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, InfraError>> + Send>>;
    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, InfraError>> + Send>>;
    fn get_user_roles_by_user_id(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, InfraError>> + Send>>;
}
