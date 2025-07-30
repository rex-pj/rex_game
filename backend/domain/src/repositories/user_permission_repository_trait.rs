use crate::{
    errors::domain_error::DomainError, models::user_permission_model::UserPermissionModel,
};
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait UserPermissionRepositoryTrait {
    fn create(
        &self,
        user_permission: UserPermissionModel,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>>;

    fn get_list_by_user_id(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, DomainError>> + Send>>;

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, DomainError>> + Send>>;
    fn create_many(
        &self,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> impl Future<Output = Result<i32, DomainError>> + Send;
    fn delete_many(
        &self,
        user_id: i32,
        user_permission_req: Vec<UserPermissionModel>,
    ) -> impl Future<Output = Result<u64, DomainError>> + Send;
}
