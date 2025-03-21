use sea_orm::DatabaseTransaction;

use super::identity_user_trait::IdentityUserTrait;
use crate::errors::application_error::ApplicationError;
use std::future::Future;

pub trait IdentityUserUseCaseTrait {
    fn create_user<UT, K>(
        &self,
        user: UT,
        password: &str,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<UT, ApplicationError>>
    where
        UT: IdentityUserTrait<K>;
}
