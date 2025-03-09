use sea_orm::{DbErr, InsertResult};
use std::future::Future;

use crate::entities::user_role;

pub trait UserRoleRepositoryTrait {
    fn create(
        &self,
        user_role: user_role::ActiveModel,
    ) -> impl Future<Output = Result<InsertResult<user_role::ActiveModel>, DbErr>>;
}
