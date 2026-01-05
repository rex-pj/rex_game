use rex_game_shared::ApplicationError;

use super::{
    user_token_creation_dto::UserTokenCreationDto, user_token_dto::UserTokenDto,
    user_token_updation_dto::UserTokenUpdationDto,
};
use std::future::Future;

pub trait IdentityUserTokenUseCaseTrait {
    fn create_user_token(
        &self,
        user_token_req: UserTokenCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn get_user_token_by_token(
        &self,
        token: &str,
    ) -> impl Future<Output = Result<UserTokenDto, ApplicationError>>;
    fn update_user_token<'a>(
        &'a self,
        id: i32,
        user_token_req: UserTokenUpdationDto,
    ) -> impl Future<Output = Result<bool, ApplicationError>>;
}
