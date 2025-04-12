use crate::errors::application_error::{ApplicationError, ErrorKind};
use chrono::Utc;
use rex_game_domain::{
    entities::{user, user_role},
    repositories::{
        role_repository_trait::RoleRepositoryTrait, user_repository_trait::UserRepositoryTrait,
        user_role_repository_trait::UserRoleRepositoryTrait,
    },
};
use sea_orm::{DatabaseTransaction, Set};

use super::{
    user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto,
    user_role_creation_dto::UserRoleCreationDto, user_role_dto::UseRoleDto,
    user_statuses::UserStatuses, user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct UserUseCase<UT, RT, URT>
where
    UT: UserRepositoryTrait,
    RT: RoleRepositoryTrait,
{
    _user_repository: UT,
    _role_repository: RT,
    _user_role_repository: URT,
}

impl<UT: UserRepositoryTrait, RT: RoleRepositoryTrait, URT: UserRoleRepositoryTrait>
    UserUseCase<UT, RT, URT>
{
    pub fn new(user_repository: UT, role_repository: RT, user_role_repository: URT) -> Self {
        Self {
            _user_repository: user_repository,
            _role_repository: role_repository,
            _user_role_repository: user_role_repository,
        }
    }
}

impl<UT: UserRepositoryTrait, RT: RoleRepositoryTrait, URT: UserRoleRepositoryTrait>
    UserUseCaseTrait for UserUseCase<UT, RT, URT>
{
    async fn get_user_by_email(&self, email: String) -> Result<UserDetailsDto, ApplicationError> {
        let existing = self._user_repository.get_by_email(email).await;
        match existing {
            Ok(i) => match i {
                Some(f) => Ok(UserDetailsDto {
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
                None => Err(ApplicationError::new(
                    ErrorKind::NotFound,
                    "User not found",
                    None,
                )),
            },
            Err(_) => Err(ApplicationError::new(
                ErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn create_user(
        &self,
        user_req: UserCreationDto,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> Result<i32, ApplicationError> {
        let active_user = user::ActiveModel {
            name: Set(user_req.name),
            display_name: Set(user_req.display_name),
            email: Set(user_req.email),
            status_id: Set(UserStatuses::Actived as i32),
            password_hash: Set(user_req.password),
            security_stamp: Set(user_req.security_stamp),
            ..Default::default()
        };

        let created = match database_transaction {
            Some(transaction) => {
                self._user_repository
                    .create_without_commit(active_user, Some(transaction))
                    .await
            }
            None => self._user_repository.create(active_user).await,
        };

        match created {
            Err(_) => Err(ApplicationError::new(
                ErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i.id),
        }
    }

    async fn get_user_roles(&self, user_id: i32) -> Result<Vec<UseRoleDto>, ApplicationError> {
        let roles = self._user_role_repository.get_user_roles(user_id).await;
        match roles {
            Ok(i) => Ok(i
                .into_iter()
                .map(|f| UseRoleDto {
                    id: f.id,
                    user_id: f.user_id,
                    role_id: f.role_id,
                    ..Default::default()
                })
                .collect()),
            Err(_) => Err(ApplicationError::new(
                ErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn assign_role(
        &self,
        user_role_req: UserRoleCreationDto,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> Result<i32, ApplicationError> {
        let role = match self
            ._role_repository
            .get_by_name(&user_role_req.role_name)
            .await
        {
            Ok(role_model) => match role_model {
                Some(role) => role,
                None => {
                    return Err(ApplicationError::new(
                        ErrorKind::NotFound,
                        "Role not found",
                        None,
                    ))
                }
            },
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Database error",
                    None,
                ))
            }
        };

        match self
            ._user_role_repository
            .create_without_commit(
                user_role::ActiveModel {
                    user_id: Set(user_role_req.user_id),
                    role_id: Set(role.id),
                    created_by_id: Set(user_role_req.created_by_id),
                    updated_by_id: Set(user_role_req.updated_by_id),
                    ..Default::default()
                },
                database_transaction,
            )
            .await
        {
            Ok(inserted) => Ok(inserted.last_insert_id),
            Err(_) => {
                return Err(ApplicationError::new(
                    ErrorKind::DatabaseError,
                    "Assign role failed",
                    None,
                ))
            }
        }
    }
}
