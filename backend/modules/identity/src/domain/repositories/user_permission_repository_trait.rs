use crate::domain::models::user_permission_model::UserPermissionModel;
use rex_game_shared::InfraError;
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait UserPermissionRepositoryTrait {
    fn create(
        &self,
        user_permission: UserPermissionModel,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, InfraError>> + Send>>;

    fn get_list_by_user_id(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, InfraError>> + Send>>;

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, InfraError>> + Send>>;
    fn create_many(
        &self,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> impl Future<Output = Result<i32, InfraError>> + Send;
    fn delete_many(
        &self,
        user_id: i32,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> impl Future<Output = Result<u64, InfraError>> + Send;
}
