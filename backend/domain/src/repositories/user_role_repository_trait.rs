use sea_orm::{DatabaseTransaction, DbErr, InsertResult};
use std::future::Future;

use crate::entities::user_role;

pub trait UserRoleRepositoryTrait {
    fn create_without_commit(
        &self,
        user_role: user_role::ActiveModel,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<InsertResult<user_role::ActiveModel>, DbErr>>;
}
