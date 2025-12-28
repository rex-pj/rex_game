use super::{
    role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto, role_dto::RoleDto,
    role_permission_creation_dto::RolePermissionCreationDto,
    role_permission_dto::RolePermissionDto, role_updation_dto::RoleUpdationDto,
    user_role_dto::UserRoleDto,
};
use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};
use std::{future::Future, pin::Pin};

pub trait RoleUseCaseTrait {
    fn get_role_by_id(&self, id: i32) -> impl Future<Output = Result<RoleDto, ApplicationError>>;
    fn get_role_by_name(&self, name: &str) -> impl Future<Output = Option<RoleDto>>;
    fn get_roles<'a>(
        &'a self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> impl Future<Output = Result<PageListModel<RoleDto>, ApplicationError>>;
    fn get_roles_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> impl Future<Output = Result<Vec<RoleDto>, ApplicationError>>;
    fn update_role<'a>(
        &'a self,
        id: i32,
        role_req: RoleUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
    fn create_role(
        &self,
        role_req: RoleCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn delete_role_by_id(
        &self,
        id: i32,
        delete_req: RoleDeletionDto,
    ) -> impl Future<Output = Option<bool>>;
    fn get_user_roles(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleDto>, ApplicationError>> + Send>>;
    fn get_role_permissions_by_role_id(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>;
    fn get_roles_permissions_by_role_ids(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>;
    fn get_role_permissions(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>;
    fn assign_permissions(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionCreationDto>,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn unassign_permissions(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionDto>,
    ) -> impl Future<Output = Result<u64, ApplicationError>>;
}
