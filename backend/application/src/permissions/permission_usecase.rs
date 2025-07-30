use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    page_list_dto::PageListDto,
    permissions::{
        permission_creation_dto::PermissionCreationDto,
        permission_deletion_dto::PermissionDeletionDto,
        permission_updation_dto::PermissionUpdationDto,
    },
};

use super::{permission_dto::PermissionDto, permission_usecase_trait::PermissionUseCaseTrait};
use chrono::Utc;
use rex_game_domain::{
    models::permission_model::PermissionModel,
    repositories::permission_repository_trait::PermissionRepositoryTrait,
};

#[derive(Clone)]
pub struct PermissionUseCase<R>
where
    R: PermissionRepositoryTrait,
{
    _permission_repository: R,
}

impl<R: PermissionRepositoryTrait> PermissionUseCase<R> {
    pub fn new(permission_repository: R) -> Self {
        Self {
            _permission_repository: permission_repository,
        }
    }
}

impl<R: PermissionRepositoryTrait> PermissionUseCaseTrait for PermissionUseCase<R> {
    async fn get_permissions(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListDto<PermissionDto>, ApplicationError> {
        let permissions_result = self
            ._permission_repository
            .get_paged_list(name, description, page, page_size_option)
            .await;
        match permissions_result {
            Ok(i) => {
                let items = i
                    .items
                    .into_iter()
                    .map(|f| PermissionDto {
                        id: f.id,
                        name: f.name,
                        description: f.description,
                        code: f.code,
                        module: f.module,
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
                    page: page,
                    page_size,
                })
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get permissions",
                None,
            )),
        }
    }

    async fn get_permission_by_id(&self, id: i32) -> Result<PermissionDto, ApplicationError> {
        let existing = self._permission_repository.get_by_id(id).await;
        match existing {
            Ok(f) => Ok(PermissionDto {
                id: f.id,
                name: f.name,
                description: f.description,
                code: f.code,
                module: f.module,
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

    async fn get_permission_by_name(
        &self,
        name: &str,
    ) -> Result<Option<PermissionDto>, ApplicationError> {
        let existing = self
            ._permission_repository
            .get_by_name(name)
            .await
            .map_err(|_| {
                ApplicationError::new(ApplicationErrorKind::DatabaseError, "Database error", None)
            })?;
        match existing {
            Some(f) => Ok(Some(PermissionDto {
                id: f.id,
                name: f.name,
                description: f.description,
                code: f.code,
                module: f.module,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
            })),
            None => Ok(None),
        }
    }

    async fn get_permission_by_code(
        &self,
        code: &str,
    ) -> Result<Option<PermissionDto>, ApplicationError> {
        let existing = self
            ._permission_repository
            .get_by_code(code)
            .await
            .map_err(|_| {
                ApplicationError::new(ApplicationErrorKind::DatabaseError, "Database error", None)
            })?;
        match existing {
            Some(f) => Ok(Some(PermissionDto {
                id: f.id,
                name: f.name,
                description: f.description,
                code: f.code,
                module: f.module,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
            })),
            None => Ok(None),
        }
    }

    async fn get_permission_by_codes(
        &self,
        codes: Vec<String>,
    ) -> Result<Vec<PermissionDto>, ApplicationError> {
        let existing = self
            ._permission_repository
            .get_by_codes(codes)
            .await
            .map_err(|_| {
                ApplicationError::new(ApplicationErrorKind::DatabaseError, "Database error", None)
            })?;
        let items = existing
            .into_iter()
            .map(|f| PermissionDto {
                id: f.id,
                name: f.name,
                description: f.description,
                code: f.code,
                module: f.module,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
            })
            .collect();
        Ok(items)
    }

    async fn create_permission(
        &self,
        permission_req: PermissionCreationDto,
    ) -> Result<i32, ApplicationError> {
        let active_permission = PermissionModel {
            name: permission_req.name,
            code: permission_req.code,
            module: permission_req.module,
            description: permission_req.description,
            created_by_id: permission_req.created_by_id,
            updated_by_id: permission_req.updated_by_id,
            ..Default::default()
        };

        let created = self._permission_repository.create(active_permission).await;

        match created {
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i),
        }
    }

    async fn update_permission<'a>(
        &'a self,
        id: i32,
        permission_req: PermissionUpdationDto,
    ) -> Option<bool> {
        let existing = self._permission_repository.get_by_id(id).await;
        match existing {
            Ok(mut exist) => {
                match permission_req.name {
                    Some(name) => exist.name = name,
                    None => {}
                };
                match permission_req.code {
                    Some(code) => exist.code = code,
                    None => {}
                };
                match permission_req.module {
                    Some(module) => exist.module = module,
                    None => {}
                };
                match permission_req.description {
                    Some(description) => exist.description = Some(description),
                    None => {}
                };
                match permission_req.is_actived {
                    Some(is_actived) => exist.is_actived = is_actived,
                    None => {}
                };
                let updated = self._permission_repository.update(exist).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    async fn delete_permission_by_id(
        &self,
        id: i32,
        delete_req: PermissionDeletionDto,
    ) -> Option<bool> {
        let updation = PermissionUpdationDto {
            updated_by_id: delete_req.updated_by_id,
            is_actived: Some(false),
            ..Default::default()
        };
        Some(self.update_permission(id, updation).await?)
    }
}
