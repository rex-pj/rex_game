use crate::{
    errors::application_error::ApplicationError,
    page_list_dto::PageListDto,
    roles::{
        role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto,
        role_updation_dto::RoleUpdationDto,
    },
};

use super::role_dto::RoleDto;
use std::future::Future;

pub trait RoleUseCaseTrait {
    fn get_role_by_id(&self, id: i32) -> impl Future<Output = Result<RoleDto, ApplicationError>>;
    fn get_roles<'a>(
        &'a self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListDto<RoleDto>, ApplicationError>>;
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
}
