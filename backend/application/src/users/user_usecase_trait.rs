use std::future::Future;

use crate::errors::application_error::ApplicationError;

use super::{user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto};

pub trait UserUseCaseTrait {
    fn get_user_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = Result<UserDetailsDto, ApplicationError>>;

    fn create_user(
        &self,
        user_req: UserCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn assign_role(
        &self,
        user_id: i32,
        role_name: &str,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
}
