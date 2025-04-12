use super::role_dto::RoleDto;
use std::future::Future;

pub trait RoleUseCaseTrait {
    fn get_roles<'a>(
        &'a self,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Option<Vec<RoleDto>>>;
}
