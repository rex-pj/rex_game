use std::future::Future;

use crate::{
    errors::domain_error::DomainError, models::user_model::UserModel,
    transaction_manager_trait::TransactionWrapperTrait,
};

pub trait UserRepositoryTrait {
    fn create(&self, user: UserModel) -> impl Future<Output = Result<i32, DomainError>>;

    fn create_without_commit(
        &self,
        user: UserModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_email(&self, email: String) -> impl Future<Output = Result<UserModel, DomainError>>;
}
