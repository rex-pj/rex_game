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
        role_permission_dto::RolePermissionDto,
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
    },
};

#[derive(Clone)]
pub struct RoleUseCase<R, PT, RP>
where
    R: RoleRepositoryTrait,
    PT: PermissionRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
{
    _role_repository: R,
    _permission_repository: PT,
    _role_permission_repository: RP,
}

impl<R: RoleRepositoryTrait, PT: PermissionRepositoryTrait, RP: RolePermissionRepositoryTrait>
    RoleUseCase<R, PT, RP>
{
    pub fn new(
        role_repository: R,
        permission_repository: PT,
        role_permission_repository: RP,
    ) -> Self {
        Self {
            _role_repository: role_repository,
            _role_permission_repository: role_permission_repository,
            _permission_repository: permission_repository,
        }
    }
}

impl<R: RoleRepositoryTrait, PT: PermissionRepositoryTrait, RP: RolePermissionRepositoryTrait>
    RoleUseCaseTrait for RoleUseCase<R, PT, RP>
where
    RP: RolePermissionRepositoryTrait + Send + Sync + Clone + 'static,
{
    async fn get_roles(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListDto<RoleDto>, ApplicationError> {
        let roles_result = self
            ._role_repository
            .get_paged_list(name, description, page, page_size)
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

    fn get_role_permissions(
        &self,
        role_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>
    {
        let role_permission_repository = self._role_permission_repository.clone();
        Box::pin(async move {
            let permissions = role_permission_repository
                .get_role_permissions(role_id)
                .await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| RolePermissionDto {
                            id: f.id,
                            role_id: f.role_id,
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

    fn get_roles_permissions(
        &self,
        role_ids: Vec<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RolePermissionDto>, ApplicationError>> + Send>>
    {
        let role_permission_repository = self._role_permission_repository.clone();
        Box::pin(async move {
            let permissions = role_permission_repository
                .get_roles_permissions(role_ids)
                .await;
            match permissions {
                Ok(i) => {
                    return Ok(i
                        .into_iter()
                        .map(|f| RolePermissionDto {
                            id: f.id,
                            role_id: f.role_id,
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

    async fn assign_role_permission(
        &self,
        role_id: i32,
        role_permission_req: RolePermissionCreationDto,
    ) -> Result<i32, ApplicationError> {
        let permission = match self
            ._permission_repository
            .get_by_code(&role_permission_req.permission_code)
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
            ._role_permission_repository
            .create(RolePermissionModel {
                role_id: role_id,
                permission_id: permission.id,
                created_by_id: role_permission_req.created_by_id,
                updated_by_id: role_permission_req.updated_by_id,
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
}
