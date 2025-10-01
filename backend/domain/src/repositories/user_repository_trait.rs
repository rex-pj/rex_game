use std::{future::Future, pin::Pin};

use crate::{
    errors::domain_error::DomainError,
    models::{page_list_model::PageListModel, user_model::UserModel},
    transaction_manager_trait::TransactionWrapperTrait,
};

pub trait UserRepositoryTrait {
    fn get_paged_list(
        &self,
        display_name: Option<String>,
        name: Option<String>,
        email: Option<String>,
        role_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<UserModel>, DomainError>>;
    fn create(&self, user: UserModel) -> impl Future<Output = Result<i32, DomainError>>;

    fn create_without_commit(
        &self,
        user: UserModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_email(
        &self,
        email: &str,
    ) -> Pin<Box<dyn Future<Output = Result<UserModel, DomainError>> + Send>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<UserModel, DomainError>>;
    fn get_by_name(&self, name: &String) -> impl Future<Output = Result<UserModel, DomainError>>;
    fn update(&self, user_req: UserModel) -> impl Future<Output = Result<bool, DomainError>>;
}
