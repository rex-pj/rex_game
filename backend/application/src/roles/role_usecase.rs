use std::{future::Future, pin::Pin};

use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    page_list_dto::PageListDto,
    roles::{
        role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto,
        role_updation_dto::RoleUpdationDto,
    },
    users::{
        role_permission_creation_dto::RolePermissionCreationDto,
        role_permission_dto::RolePermissionDto, user_role_dto::UserRoleDto,
    },
};

use super::{role_dto::RoleDto, role_usecase_trait::RoleUseCaseTrait};
use chrono::Utc;
use rex_game_domain::{
    models::{role_model::RoleModel, role_permission_model::RolePermissionModel},
    repositories::{
        permission_repository_trait::PermissionRepositoryTrait,
        role_permission_repository_trait::RolePermissionRepositoryTrait,
        role_repository_trait::RoleRepositoryTrait,
        user_role_repository_trait::UserRoleRepositoryTrait,
    },
};

#[derive(Clone)]
pub struct RoleUseCase<R, PT, RP, UR>
where
    R: RoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait,
{
    _role_repository: R,
    _permission_repository: PT,
    _role_permission_repository: RP,
    _user_role_repository: UR,
}

impl<R, PT, RP, UR> RoleUseCase<R, PT, RP, UR>
where
    R: RoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait,
{
    pub fn new(
        role_repository: R,
        permission_repository: PT,
        role_permission_repository: RP,
        user_role_repository: UR,
    ) -> Self {
        Self {
            _role_repository: role_repository,
            _role_permission_repository: role_permission_repository,
            _permission_repository: permission_repository,
            _user_role_repository: user_role_repository,
        }
    }
}

impl<R, PT, RP, UR> RoleUseCaseTrait for RoleUseCase<R, PT, RP, UR>
where
    RP: RolePermissionRepositoryTrait + Send + Sync + Clone + 'static,
    R: RoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait + Send + Sync + Clone + 'static,
{
    async fn get_roles(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListDto<RoleDto>, ApplicationError> {
        let roles_result = self
            ._role_repository
            .get_paged_list(name, description, page, page_size_option)
            .await;
        match roles_result {
            Ok(i) => {
                let items = i
                    .items
                    .into_iter()
                    .map(|f| RoleDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                    })
                    .collect();
                let page_size: u64 = page_size_option.unwrap_or(i.total_count);
                Ok(PageListDto {
                    items,
                    total_count: i.total_count,
                    page,
                    page_size,
                })
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get roles",
                None,
            )),
        }
    }

    async fn get_roles_by_ids(&self, ids: Vec<i32>) -> Result<Vec<RoleDto>, ApplicationError> {
        let roles_result = self._role_repository.get_by_ids(ids).await;
        match roles_result {
            Ok(i) => {
                let items = i
                    .into_iter()
                    .map(|f| RoleDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                    })
                    .collect();
                Ok(items)
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get roles by ids",
                None,
            )),
        }
    }

    async fn get_role_by_name(&self, name: &str) -> Option<RoleDto> {
        let existing = self._role_repository.get_by_name(name).await;
        match existing {
            Some(f) => Some(RoleDto {
                id: f.id,
                name: f.name,
                description: f.description,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
            }),
            None => None,
        }
    }

    async fn get_role_by_id(&self, id: i32) -> Result<RoleDto, ApplicationError> {
        let existing = self._role_repository.get_by_id(id).await;
        match existing {
            Ok(f) => Ok(RoleDto {
                id: f.id,
                name: f.name,
                description: f.description,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
            }),
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn create_role(&self, role_req: RoleCreationDto) -> Result<i32, ApplicationError> {
        let active_role = RoleModel {
            name: role_req.name,
            description: role_req.description,
            created_by_id: role_req.created_by_id,
            updated_by_id: role_req.updated_by_id,
            ..Default::default()
        };

        let created = self._role_repository.create(active_role).await;

        match created {
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i),
        }
    }

    async fn update_role<'a>(&'a self, id: i32, role_req: RoleUpdationDto) -> Option<bool> {
        let existing = self._role_repository.get_by_id(id).await;
        match existing {
            Ok(mut exist) => {
                match role_req.name {
                    Some(name) => exist.name = name,
                    None => {}
                };
                match role_req.description {
                    Some(description) => exist.description = Some(description),
                    None => {}
                };
                match role_req.is_actived {
                    Some(is_actived) => exist.is_actived = is_actived,
                    None => {}
                };
                let updated = self._role_repository.update(exist).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    async fn delete_role_by_id(&self, id: i32, delete_req: RoleDeletionDto) -> Option<bool> {
        let updation = RoleUpdationDto {
            updated_by_id: delete_req.updated_by_id,
            is_actived: Some(false),
            ..Default::default()
        };
        Some(self.update_role(id, updation).await?)
    }

    fn get_user_roles(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleDto>, ApplicationError>> + Send>> {
        let user_role_repository = self._user_role_repository.clone();
        Box::pin(async move {
            let roles = user_role_repository.get_list().await;
            match roles {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| UserRoleDto {
                            id: f.id,
                            user_id: f.user_id,
                            user_name: f.user_name,
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

    fn get_role_permissions_by_role_id(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>
    {
        let role_permission_repository = self._role_permission_repository.clone();
        Box::pin(async move {
            let permissions = role_permission_repository
                .get_list_by_role_id(role_id)
                .await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| RolePermissionDto {
                            id: f.id,
                            role_id: f.role_id,
                            role_name: f.role_name,
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

    fn get_roles_permissions_by_role_ids(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>
    {
        let role_permission_repository = self._role_permission_repository.clone();
        Box::pin(async move {
            let permissions = role_permission_repository
                .get_list_by_role_ids(role_ids)
                .await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| RolePermissionDto {
                            id: f.id,
                            role_id: f.role_id,
                            role_name: f.role_name,
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

    fn get_role_permissions(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>
    {
        let role_permission_repository = self._role_permission_repository.clone();
        Box::pin(async move {
            let permissions = role_permission_repository.get_list().await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| RolePermissionDto {
                            id: f.id,
                            role_id: f.role_id,
                            role_name: f.role_name,
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

    async fn assign_permissions(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionCreationDto>,
    ) -> Result<i32, ApplicationError> {
        let role_permissions = role_permission_req
            .into_iter()
            .map(|f| RolePermissionModel {
                role_id: role_id,
                permission_id: f.permission_id,
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                ..Default::default()
            })
            .collect::<Vec<RolePermissionModel>>();
        match self
            ._role_permission_repository
            .create_many(role_permissions)
            .await
        {
            Ok(inserted) => Ok(inserted),
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Assign permission failed",
                    None,
                ))
            }
        }
    }

    async fn unassign_permissions(
        &self,
        role_id: i32,
        role_permission_req: Vec<RolePermissionDto>,
    ) -> Result<u64, ApplicationError> {
        let deleted_permissions = role_permission_req
            .into_iter()
            .map(|f| RolePermissionModel {
                permission_id: f.permission_id,
                ..Default::default()
            })
            .collect::<Vec<RolePermissionModel>>();
        match self
            ._role_permission_repository
            .delete_many(role_id, deleted_permissions)
            .await
        {
            Ok(deleted) => Ok(deleted),
            Err(_) => {
                return Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    "Assign permission failed",
                    None,
                ))
            }
        }
    }
}
