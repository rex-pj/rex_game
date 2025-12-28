use super::{
    permission_creation_dto::PermissionCreationDto, permission_deletion_dto::PermissionDeletionDto,
    permission_dto::PermissionDto, permission_updation_dto::PermissionUpdationDto,
    permission_usecase_trait::PermissionUseCaseTrait,
};
use crate::{
    domain::models::permission_model::PermissionModel,
    domain::repositories::permission_repository_trait::PermissionRepositoryTrait,
};
use chrono::Utc;
use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};

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
    ) -> Result<PageListModel<PermissionDto>, ApplicationError> {
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
                Ok(PageListModel {
                    items,
                    total_count: i.total_count,
                })
            }
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
            .map_err(|err| ApplicationError::Infrastructure(err))?;
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
            .map_err(|err| ApplicationError::Infrastructure(err))?;
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
            .map_err(|err| ApplicationError::Infrastructure(err))?;
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
            Err(err) => Err(ApplicationError::Infrastructure(err)),
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
