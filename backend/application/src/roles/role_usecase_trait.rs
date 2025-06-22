use crate::{
    errors::application_error::ApplicationError, roles::role_updation_dto::RoleUpdationDto,
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
    ) -> impl Future<Output = Option<Vec<RoleDto>>>;
    fn update_role<'a>(
        &'a self,
        id: i32,
        role_req: RoleUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
}
