use super::{role_dto::RoleDto, role_usecase_trait::RoleUseCaseTrait};
use chrono::Utc;
use rex_game_domain::repositories::role_repository_trait::RoleRepositoryTrait;

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
    async fn get_roles(&self, page: u64, page_size: u64) -> Option<Vec<RoleDto>> {
        let roles_result = self._role_repository.get_list(page, page_size).await;
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
}
