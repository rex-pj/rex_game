use rex_game_shared::domain::transaction_manager_trait::TransactionWrapperTrait;
use rex_game_shared::ApplicationError;
use std::future::Future;
use std::pin::Pin;

use super::identity_user_trait::IdentityUserTrait;
use crate::application::usecases::loggedin_user_dto::LoggedInUserDto;

pub trait IdentityUserUseCaseTrait {
    fn create_user_with_transaction<UT, K>(
        &self,
        user: UT,
        password: &str,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<UT, ApplicationError>>
    where
        UT: IdentityUserTrait<K>;

    fn create_user<UT, K>(
        &self,
        user: UT,
        password: &str,
    ) -> impl Future<Output = Result<UT, ApplicationError>>
    where
        UT: IdentityUserTrait<K>;

    fn get_logged_in_user<'a>(
        &'a self,
        access_token: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<LoggedInUserDto, ApplicationError>> + Send + 'a>>;
}
