use crate::{
    errors::application_error::{ApplicationError, ErrorKind},
    roles::role_updation_dto::RoleUpdationDto,
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
    ) -> Option<Vec<RoleDto>> {
        let roles_result = self
            ._role_repository
            .get_paged_list(name, description, page, page_size)
            .await;
        let roles = match roles_result {
            Ok(list) => Some(
                list.items
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
                    .collect(),
            ),
            Err(_) => Some(Vec::new()),
        };

        roles
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

    async fn update_role<'a>(&'a self, id: i32, role_req: RoleUpdationDto) -> Option<bool> {
        let existing = self._role_repository.get_by_id(id).await;
        match existing {
            Ok(exist) => {
                let updating = RoleModel {
                    id: exist.id,
                    name: role_req.name,
                    description: role_req.description,
                    ..Default::default()
                };
                let updated = self._role_repository.update(updating).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }
}
