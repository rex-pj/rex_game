use sea_orm::DbErr;
use std::future::Future;

use crate::entities::role;

pub trait RoleRepositoryTrait {
    fn create(&self, role: role::ActiveModel) -> impl Future<Output = Result<role::Model, DbErr>>;

    fn get_by_name(&self, name: &str) -> impl Future<Output = Result<Option<role::Model>, DbErr>>;
    fn get_list(
        &self,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<(Vec<role::Model>, u64), DbErr>>;
}
