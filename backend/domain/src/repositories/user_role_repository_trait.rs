use crate::{
    errors::domain_error::DomainError, models::user_role_model::UserRoleModel,
    transaction_manager_trait::TransactionWrapperTrait,
};
use std::{collections::HashSet, future::Future, pin::Pin};

pub trait UserRoleRepositoryTrait {
    fn create_without_commit(
        &self,
        user_role: UserRoleModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, DomainError>>;
    fn create(
        &self,
        user_role_req: UserRoleModel,
    ) -> impl Future<Output = Result<i32, DomainError>> + Send;
    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>>;

    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, DomainError>> + Send>>;
}
