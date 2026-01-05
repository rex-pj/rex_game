use rex_game_shared::InfraError;
use std::future::Future;

use crate::domain::models::permission_model::PermissionModel;
use rex_game_shared::domain::models::page_list_model::PageListModel;

pub trait PermissionRepositoryTrait {
    fn create(&self, permission: PermissionModel) -> impl Future<Output = Result<i32, InfraError>>;

    fn get_by_code(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<PermissionModel>, InfraError>>;
    fn get_by_codes(
        &self,
        codes: Vec<String>,
    ) -> impl Future<Output = Result<Vec<PermissionModel>, InfraError>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<PermissionModel, InfraError>>;
    fn get_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<PermissionModel>, InfraError>>;
    fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> impl Future<Output = Result<PageListModel<PermissionModel>, InfraError>>;
    fn update(
        &self,
        permission_req: PermissionModel,
    ) -> impl Future<Output = Result<bool, InfraError>>;
}
