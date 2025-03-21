use sea_orm::{DatabaseTransaction, DbErr};
use std::future::Future;

use crate::entities::user;

pub trait UserRepositoryTrait {
    fn create(&self, user: user::ActiveModel) -> impl Future<Output = Result<user::Model, DbErr>>;

    fn create_without_commit(
        &self,
        user: user::ActiveModel,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<user::Model, DbErr>>;

    fn get_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = Result<Option<user::Model>, DbErr>>;
}
