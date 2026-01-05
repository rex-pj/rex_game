use super::{
    permission_creation_dto::PermissionCreationDto, permission_deletion_dto::PermissionDeletionDto,
    permission_dto::PermissionDto, permission_updation_dto::PermissionUpdationDto,
};
use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};
use std::future::Future;

pub trait PermissionUseCaseTrait {
    fn get_permission_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<PermissionDto, ApplicationError>>;
    fn get_permission_by_code(
        &self,
        code: &str,
    ) -> impl Future<Output = Result<Option<PermissionDto>, ApplicationError>>;
    fn get_permission_by_codes(
        &self,
        codes: Vec<String>,
    ) -> impl Future<Output = Result<Vec<PermissionDto>, ApplicationError>>;
    fn get_permission_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<PermissionDto>, ApplicationError>>;
    fn get_permissions<'a>(
        &'a self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> impl Future<Output = Result<PageListModel<PermissionDto>, ApplicationError>>;
    fn update_permission<'a>(
        &'a self,
        id: i32,
        permission_req: PermissionUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
    fn create_permission(
        &self,
        permission_req: PermissionCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn delete_permission_by_id(
        &self,
        id: i32,
        delete_req: PermissionDeletionDto,
    ) -> impl Future<Output = Option<bool>>;
}
