use std::future::Future;

use crate::{
    errors::domain_error::DomainError,
    models::{page_list_model::PageListModel, permission_model::PermissionModel},
};

pub trait PermissionRepositoryTrait {
    fn create(&self, permission: PermissionModel)
        -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_code(&self, name: &str)
        -> impl Future<Output = Result<PermissionModel, DomainError>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<PermissionModel, DomainError>>;
    fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<PermissionModel>, DomainError>>;
    fn update(
        &self,
        permission_req: PermissionModel,
    ) -> impl Future<Output = Result<bool, DomainError>>;
}
