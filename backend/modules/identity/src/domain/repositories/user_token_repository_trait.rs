use crate::domain::models::user_token_model::UserTokenModel;
use rex_game_shared::InfraError;
use std::future::Future;

pub trait UserTokenRepositoryTrait {
    fn create(&self, user_token: UserTokenModel) -> impl Future<Output = Result<i32, InfraError>>;
    fn get_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<UserTokenModel, InfraError>> + Send;
    fn get_by_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<UserTokenModel, InfraError>>;
    fn update(
        &self,
        user_req: UserTokenModel,
    ) -> impl Future<Output = Result<bool, InfraError>> + Send;
}
