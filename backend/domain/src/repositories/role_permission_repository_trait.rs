use crate::{
    errors::domain_error::DomainError, models::role_permission_model::RolePermissionModel,
};
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait RolePermissionRepositoryTrait {
    fn create(
        &self,
        role_permission: RolePermissionModel,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn are_roles_in_permission(
        &self,
        role_ids: Vec<i32>,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>>;

    fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>>;
    fn get_roles_permissions(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionModel>, DomainError>> + Send>>;
}
