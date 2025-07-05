use std::{future::Future, pin::Pin};

use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    page_list_dto::PageListDto,
    users::{
        user_deletion_dto::UserDeletionDto, user_dto::UserDto,
        user_permission_creation_dto::UserPermissionCreationDto,
        user_permission_dto::UserPermissionDto, user_updation_dto::UserUpdationDto,
    },
};
use chrono::Utc;
use rex_game_domain::{
    models::{
        user_model::UserModel, user_permission_model::UserPermissionModel,
        user_role_model::UserRoleModel, user_statuses::UserStatuses,
    },
    repositories::{
        permission_repository_trait::PermissionRepositoryTrait,
        role_repository_trait::RoleRepositoryTrait,
        user_permission_repository_trait::UserPermissionRepositoryTrait,
        user_repository_trait::UserRepositoryTrait,
        user_role_repository_trait::UserRoleRepositoryTrait,
    },
    transaction_manager_trait::TransactionWrapperTrait,
};

use super::{
    user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto,
    user_role_creation_dto::UserRoleCreationDto, user_role_dto::UserRoleDto,
    user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct UserUseCase<UT, RT, URT, PT, UP>
where
    UT: UserRepositoryTrait,
    RT: RoleRepositoryTrait,
    URT: UserRoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    UP: UserPermissionRepositoryTrait,
{
    _user_repository: UT,
    _role_repository: RT,
    _user_role_repository: URT,
    _permission_repository: PT,
    _user_permission_repository: UP,
}

impl<UT, RT, URT, PT, UP> UserUseCase<UT, RT, URT, PT, UP>
where
    UT: UserRepositoryTrait,
    RT: RoleRepositoryTrait,
    URT: UserRoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    UP: UserPermissionRepositoryTrait,
{
    pub fn new(
        user_repository: UT,
        role_repository: RT,
        user_role_repository: URT,
        permission_repository: PT,
        user_permission_repository: UP,
    ) -> Self {
        Self {
            _user_repository: user_repository,
            _role_repository: role_repository,
            _user_role_repository: user_role_repository,
            _permission_repository: permission_repository,
            _user_permission_repository: user_permission_repository,
        }
    }
}

impl<UT, RT, URT, PT, UP> UserUseCaseTrait for UserUseCase<UT, RT, URT, PT, UP>
where
    UT: UserRepositoryTrait,
    RT: RoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    URT: UserRoleRepositoryTrait + Send + Sync + Clone + 'static,
    UP: UserPermissionRepositoryTrait + Send + Sync + Clone + 'static,
{
    async fn get_user_by_email(&self, email: String) -> Result<UserDetailsDto, ApplicationError> {
        let existing = self._user_repository.get_by_email(email).await;
        match existing {
            Ok(f) => Ok(UserDetailsDto {
                id: f.id,
                email: f.email,
                name: f.name,
                display_name: f.display_name,
                password_hash: f.password_hash,
                security_stamp: f.security_stamp,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
                status_id: f.status_id,
            }),
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn get_user_by_id(&self, id: i32) -> Result<UserDto, ApplicationError> {
        let existing = self._user_repository.get_by_id(id).await;
        match existing {
            Ok(f) => Ok(UserDto {
                id: f.id,
                email: f.email,
                name: f.name,
                display_name: f.display_name,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
                status_id: f.status_id,
            }),
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn create_user_with_transaction(
        &self,
        user_req: UserCreationDto,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<i32, ApplicationError> {
        let active_user = UserModel {
            name: user_req.name,
            display_name: user_req.display_name,
            email: user_req.email,
            status_id: UserStatuses::Actived as i32,
            password_hash: user_req.password,
            security_stamp: user_req.security_stamp,
            ..Default::default()
        };

        let created = self
            ._user_repository
            .create_without_commit(active_user, transaction)
            .await;

        match created {
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i),
        }
    }

    async fn get_users<'a>(
        &'a self,
        display_name: Option<String>,
        name: Option<String>,
        email: Option<String>,
        role_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListDto<UserDto>, ApplicationError> {
        match self
            ._user_repository
            .get_paged_list(display_name, name, email, role_name, page, page_size)
            .await
        {
            Ok(i) => {
                let items = i
                    .items
                    .into_iter()
                    .map(|f| UserDto {
                        id: f.id,
                        name: f.name,
                        created_by_id: f.created_by_id,
                        display_name: f.display_name,
                        email: f.email,
                        status_id: f.status_id,
                        updated_by_id: f.updated_by_id,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                    })
                    .collect();
                Ok(PageListDto {
                    items,
                    total_count: i.total_count,
                    page,
                    page_size,
                })
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get users",
                None,
            )),
        }
    }

    async fn create_user(&self, user_req: UserCreationDto) -> Result<i32, ApplicationError> {
        let active_user = UserModel {
            name: user_req.name,
            display_name: user_req.display_name,
            email: user_req.email,
            status_id: UserStatuses::Actived as i32,
            password_hash: user_req.password,
            security_stamp: user_req.security_stamp,
            ..Default::default()
        };

        let created = self._user_repository.create(active_user).await;

        match created {
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i),
        }
    }

    async fn update_user<'a>(&'a self, id: i32, user_req: UserUpdationDto) -> Option<bool> {
        let existing = self._user_repository.get_by_id(id).await;
        match existing {
            Ok(mut exist) => {
                exist.updated_by_id = user_req.updated_by_id;
                match user_req.name {
                    Some(name) => exist.name = name,
                    None => {}
                };
                match user_req.email {
                    Some(email) => exist.email = email,
                    None => {}
                };
                match user_req.display_name {
                    Some(display_name) => exist.display_name = Some(display_name),
                    None => {}
                };
                match user_req.status_id {
                    Some(status_id) => exist.status_id = status_id,
                    None => {}
                };

                exist.updated_by_id = user_req.updated_by_id;
                let updated = self._user_repository.update(exist).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    async fn delete_user_by_id(&self, id: i32, delete_req: UserDeletionDto) -> Option<bool> {
        let updation = UserUpdationDto {
            status_id: Some(UserStatuses::Deleted as i32),
            updated_by_id: delete_req.updated_by_id,
            ..Default::default()
        };
        Some(self.update_user(id, updation).await?)
    }

    async fn assign_role_with_transaction(
        &self,
        user_id: i32,
        user_role_req: UserRoleCreationDto,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<i32, ApplicationError> {
        let role = match self
            ._role_repository
            .get_by_name(&user_role_req.role_name)
            .await
        {
            Ok(role_model) => role_model,
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Database error",
                    None,
                ))
            }
        };

        match self
            ._user_role_repository
            .create_without_commit(
                UserRoleModel {
                    user_id: user_id,
                    role_id: role.id,
                    created_by_id: user_role_req.created_by_id,
                    updated_by_id: user_role_req.updated_by_id,
                    ..Default::default()
                },
                transaction,
            )
            .await
        {
            Ok(inserted) => Ok(inserted),
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Assign role failed",
                    None,
                ))
            }
        }
    }

    async fn assign_role(
        &self,
        user_id: i32,
        user_role_req: UserRoleCreationDto,
    ) -> Result<i32, ApplicationError> {
        let role = match self
            ._role_repository
            .get_by_name(&user_role_req.role_name)
            .await
        {
            Ok(role_model) => role_model,
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Database error",
                    None,
                ))
            }
        };

        match self
            ._user_role_repository
            .create(UserRoleModel {
                user_id: user_id,
                role_id: role.id,
                created_by_id: user_role_req.created_by_id,
                updated_by_id: user_role_req.updated_by_id,
                ..Default::default()
            })
            .await
        {
            Ok(inserted) => Ok(inserted),
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Assign role failed",
                    None,
                ))
            }
        }
    }

    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleDto>, ApplicationError>> + Send>> {
        let user_role_repository = self._user_role_repository.clone();
        Box::pin(async move {
            let roles = user_role_repository.get_user_roles(user_id).await;
            match roles {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| UserRoleDto {
                            id: f.id,
                            user_id: f.user_id,
                            role_id: f.role_id,
                            role_name: f.role_name,
                            ..Default::default()
                        })
                        .collect());
                }
                Err(_) => Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Database error",
                    None,
                )),
            }
        })
    }

    async fn assign_user_permission(
        &self,
        user_id: i32,
        user_permission_req: UserPermissionCreationDto,
    ) -> Result<i32, ApplicationError> {
        let permission = match self
            ._permission_repository
            .get_by_code(&user_permission_req.permission_code)
            .await
        {
            Ok(permission_model) => permission_model,
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Database error",
                    None,
                ))
            }
        };

        match self
            ._user_permission_repository
            .create(UserPermissionModel {
                user_id: user_id,
                permission_id: permission.id,
                created_by_id: user_permission_req.created_by_id,
                updated_by_id: user_permission_req.updated_by_id,
                ..Default::default()
            })
            .await
        {
            Ok(inserted) => Ok(inserted),
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Assign role failed",
                    None,
                ))
            }
        }
    }

    fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionDto>, ApplicationError>> + Send>>
    {
        let user_permission_repository = self._user_permission_repository.clone();
        Box::pin(async move {
            let permissions = user_permission_repository
                .get_user_permissions(user_id)
                .await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| UserPermissionDto {
                            id: f.id,
                            user_id: f.user_id,
                            permission_id: f.permission_id,
                            permission_name: f.permission_name,
                            permission_code: f.permission_code,
                            permission_module: f.permission_module,
                            ..Default::default()
                        })
                        .collect());
                }
                Err(_) => Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Database error",
                    None,
                )),
            }
        })
    }
}
