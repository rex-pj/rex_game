use rex_game_shared::InfraError;
use std::{future::Future, pin::Pin};

use crate::domain::models::user_model::UserModel;
use rex_game_shared::domain::{
    models::page_list_model::PageListModel, transaction_manager_trait::TransactionWrapperTrait,
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
    ) -> impl Future<Output = Result<PageListModel<UserModel>, InfraError>>;
    fn create(&self, user: UserModel) -> impl Future<Output = Result<i32, InfraError>>;

    fn create_without_commit(
        &self,
        user: UserModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn get_by_email(
        &self,
        email: &str,
    ) -> Pin<Box<dyn Future<Output = Result<UserModel, InfraError>> + Send>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<UserModel, InfraError>>;
    fn get_by_name(&self, name: &String) -> impl Future<Output = Result<UserModel, InfraError>>;
    fn update(&self, user_req: UserModel) -> impl Future<Output = Result<bool, InfraError>>;
}
