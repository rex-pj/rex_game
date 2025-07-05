use rex_game_domain::transaction_manager_trait::TransactionWrapperTrait;

use crate::{
    errors::application_error::ApplicationError,
    page_list_dto::PageListDto,
    users::{
        user_deletion_dto::UserDeletionDto, user_dto::UserDto,
        user_permission_creation_dto::UserPermissionCreationDto,
        user_permission_dto::UserPermissionDto, user_updation_dto::UserUpdationDto,
    },
};
use std::{future::Future, pin::Pin};

use super::{
    user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto,
    user_role_creation_dto::UserRoleCreationDto, user_role_dto::UserRoleDto,
};

pub trait UserUseCaseTrait {
    fn get_user_by_email(
        &self,
        email: String,
    ) -> impl Future<Output = Result<UserDetailsDto, ApplicationError>>;

    fn get_user_by_id(&self, id: i32) -> impl Future<Output = Result<UserDto, ApplicationError>>;

    fn get_users<'a>(
        &'a self,
        display_name: Option<String>,
        name: Option<String>,
        email: Option<String>,
        role_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListDto<UserDto>, ApplicationError>>;

    fn create_user_with_transaction(
        &self,
        user_req: UserCreationDto,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;

    fn create_user(
        &self,
        user_req: UserCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn assign_role_with_transaction(
        &self,
        user_id: i32,
        user_role_req: UserRoleCreationDto,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn assign_role(
        &self,
        user_id: i32,
        user_role_req: UserRoleCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleDto>, ApplicationError>> + Send>>;
    fn update_user<'a>(
        &'a self,
        id: i32,
        user_req: UserUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
    fn delete_user_by_id(
        &self,
        id: i32,
        user_req: UserDeletionDto,
    ) -> impl Future<Output = Option<bool>>;
    fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionDto>, ApplicationError>> + Send>>;

    fn assign_user_permission(
        &self,
        user_id: i32,
        user_permission_req: UserPermissionCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
}
