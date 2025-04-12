use sea_orm::DatabaseTransaction;

use crate::errors::application_error::ApplicationError;
use std::future::Future;

use super::{
    user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto,
    user_role_creation_dto::UserRoleCreationDto, user_role_dto::UseRoleDto,
};

pub trait UserUseCaseTrait {
    fn get_user_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = Result<UserDetailsDto, ApplicationError>>;

    fn create_user(
        &self,
        user_req: UserCreationDto,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn assign_role(
        &self,
        user_role_req: UserRoleCreationDto,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> impl Future<Output = Result<Vec<UseRoleDto>, ApplicationError>>;
}
