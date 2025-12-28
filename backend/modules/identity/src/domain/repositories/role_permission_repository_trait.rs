use crate::domain::models::role_permission_model::RolePermissionModel;
use rex_game_shared::InfraError;
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait RolePermissionRepositoryTrait {
    fn create(
        &self,
        role_permission: RolePermissionModel,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn are_roles_in_permission(
        &self,
        role_ids: Vec<i32>,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, InfraError>> + Send>>;

    fn get_list_by_role_id(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, InfraError>> + Send>>;
    fn get_list_by_role_ids(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, InfraError>> + Send>>;

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, InfraError>> + Send>>;
    fn create_many(
        &self,
        role_permission_req: Vec<RolePermissionModel>,
    ) -> impl Future<Output = Result<i32, InfraError>> + Send;
    fn delete_many(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionModel>,
    ) -> impl Future<Output = Result<u64, InfraError>> + Send;
}
