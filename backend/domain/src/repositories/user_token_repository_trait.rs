use crate::{errors::domain_error::DomainError, models::user_token_model::UserTokenModel};
use std::future::Future;

pub trait UserTokenRepositoryTrait {
    fn create(&self, user_token: UserTokenModel) -> impl Future<Output = Result<i32, DomainError>>;
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<UserTokenModel, DomainError>> + Send;
    fn get_by_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<UserTokenModel, DomainError>>;
    fn update(
        &self,
        user_req: UserTokenModel,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
}
