use std::future::Future;

use crate::errors::application_error::ApplicationError;

use super::identity_user_trait::IdentityUserTrait;

pub trait IdentityUserUseCaseTrait {
    fn create_user<UT, K>(
        &self,
        user: UT,
        password: &str,
    ) -> impl Future<Output = Result<UT, ApplicationError>>
    where
        UT: IdentityUserTrait<K>;
}
