use std::{collections::HashSet, future::Future, pin::Pin};

use crate::errors::application_error::ApplicationError;

pub trait IdentityAuthorizeUseCaseTrait {
    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ApplicationError>> + Send>>;
}
