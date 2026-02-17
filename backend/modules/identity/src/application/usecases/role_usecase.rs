use super::{
    role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto, role_dto::RoleDto,
    role_permission_creation_dto::RolePermissionCreationDto,
    role_permission_dto::RolePermissionDto, role_updation_dto::RoleUpdationDto,
    role_usecase_trait::RoleUseCaseTrait, user_role_dto::UserRoleDto,
};
use crate::domain::{
    models::{role_model::RoleModel, role_permission_model::RolePermissionModel},
    repositories::{
        role_permission_repository_trait::RolePermissionRepositoryTrait,
        role_repository_trait::RoleRepositoryTrait,
        user_role_repository_trait::UserRoleRepositoryTrait,
    },
};
use chrono::Utc;
use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};
use std::{future::Future, pin::Pin};

#[derive(Clone)]
pub struct RoleUseCase<R, RP, UR>
where
    R: RoleRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait,
{
    _role_repository: R,
    _role_permission_repository: RP,
    _user_role_repository: UR,
}

impl<R, RP, UR> RoleUseCase<R, RP, UR>
where
    R: RoleRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait,
{
    pub fn new(
        role_repository: R,
        role_permission_repository: RP,
        user_role_repository: UR,
    ) -> Self {
        Self {
            _role_repository: role_repository,
            _role_permission_repository: role_permission_repository,
            _user_role_repository: user_role_repository,
        }
    }
}

impl<R, RP, UR> RoleUseCaseTrait for RoleUseCase<R, RP, UR>
where
    RP: RolePermissionRepositoryTrait + Send + Sync + Clone + 'static,
    R: RoleRepositoryTrait,
    RP: RolePermissionRepositoryTrait,
    UR: UserRoleRepositoryTrait + Send + Sync + Clone + 'static,
{
    async fn get_roles(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListModel<RoleDto>, ApplicationError> {
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
                        created_on: f.created_on.with_timezone(&Utc),
                        updated_on: f.updated_on.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                    })
                    .collect();
                Ok(PageListModel {
                    items,
                    total_count: i.total_count,
                })
            }
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                        created_on: f.created_on.with_timezone(&Utc),
                        updated_on: f.updated_on.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                    })
                    .collect();
                Ok(items)
            }
            Err(err) => Err(ApplicationError::Infrastructure(err)),
        }
    }

    async fn get_role_by_name(&self, name: &str) -> Option<RoleDto> {
        let existing = self._role_repository.get_by_name(name).await;
        match existing {
            Some(f) => Some(RoleDto {
                id: f.id,
                name: f.name,
                description: f.description,
                created_on: f.created_on.with_timezone(&Utc),
                updated_on: f.updated_on.with_timezone(&Utc),
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
                created_on: f.created_on.with_timezone(&Utc),
                updated_on: f.updated_on.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
            }),
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                Err(err) => Err(ApplicationError::Infrastructure(err)),
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
                Err(err) => Err(ApplicationError::Infrastructure(err)),
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
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
            Err(err) => Err(ApplicationError::Infrastructure(err)),
        }
    }
}
