use crate::{
    errors::application_error::{ApplicationError, ErrorKind},
    page_list_dto::PageListDto,
    roles::{
        role_creation_dto::RoleCreationDto, role_deletion_dto::RoleDeletionDto,
        role_updation_dto::RoleUpdationDto,
    },
};

use super::{role_dto::RoleDto, role_usecase_trait::RoleUseCaseTrait};
use chrono::Utc;
use rex_game_domain::{
    models::role_model::RoleModel, repositories::role_repository_trait::RoleRepositoryTrait,
};

#[derive(Clone)]
pub struct RoleUseCase<R>
where
    R: RoleRepositoryTrait,
{
    _role_repository: R,
}

impl<R: RoleRepositoryTrait> RoleUseCase<R> {
    pub fn new(role_repository: R) -> Self {
        Self {
            _role_repository: role_repository,
        }
    }
}

impl<R: RoleRepositoryTrait> RoleUseCaseTrait for RoleUseCase<R> {
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
                ErrorKind::DatabaseError,
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
                ErrorKind::DatabaseError,
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
                ErrorKind::DatabaseError,
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
}
