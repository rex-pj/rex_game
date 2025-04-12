use sea_orm::{DatabaseTransaction, DbErr, InsertResult};
use std::{collections::HashSet, future::Future, pin::Pin};

use crate::entities::user_role;

pub trait UserRoleRepositoryTrait {
    fn create_without_commit(
        &self,
        user_role: user_role::ActiveModel,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<InsertResult<user_role::ActiveModel>, DbErr>>;

    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DbErr>> + Send>>;

    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<user_role::Model>, DbErr>> + Send>>;
}
