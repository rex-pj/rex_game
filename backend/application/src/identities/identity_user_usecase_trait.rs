use rex_game_domain::transaction_manager_trait::TransactionWrapperTrait;

use super::identity_user_trait::IdentityUserTrait;
use crate::{
    errors::application_error::ApplicationError, users::loggedin_user_dto::LoggedInUserDto,
};
use std::future::Future;

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

    fn get_logged_in_user(
        &self,
        access_token: &str,
    ) -> impl Future<Output = Result<LoggedInUserDto, ApplicationError>>;
}
