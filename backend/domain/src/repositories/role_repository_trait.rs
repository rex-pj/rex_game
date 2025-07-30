use std::future::Future;

use crate::{
    errors::domain_error::DomainError,
    models::{page_list_model::PageListModel, role_model::RoleModel},
};

pub trait RoleRepositoryTrait {
    fn create(&self, role: RoleModel) -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_name(&self, name: &str) -> impl Future<Output = Option<RoleModel>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<RoleModel, DomainError>>;
    fn get_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> impl Future<Output = Result<Vec<RoleModel>, DomainError>>;
    fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page_option: u64,
        page_size: Option<u64>,
    ) -> impl Future<Output = Result<PageListModel<RoleModel>, DomainError>>;
    fn update(&self, role_req: RoleModel) -> impl Future<Output = Result<bool, DomainError>>;
}
