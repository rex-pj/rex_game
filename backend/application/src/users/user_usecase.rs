use chrono::Utc;
use rex_game_domain::{entities::user, repositories::user_repository_trait::UserRepositoryTrait};
use sea_orm::{DbErr, Set};

use super::{
    user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto,
    user_login_parameter::UserLoginParameter, user_statuses::UserStatuses,
    user_usecase_trait::UserUseCaseTrait,
};

#[derive(Clone)]
pub struct UserUseCase<UT>
where
    UT: UserRepositoryTrait,
{
    _user_repository: UT,
}

impl<UT: UserRepositoryTrait> UserUseCase<UT> {
    pub fn new(user_repository: UT) -> Self {
        Self {
            _user_repository: user_repository,
        }
    }
}

impl<UT: UserRepositoryTrait> UserUseCaseTrait for UserUseCase<UT> {
    async fn get_user_by_email<'a>(
        &'a self,
        parameters: UserLoginParameter,
    ) -> Result<UserDetailsDto, DbErr> {
        let existing = self._user_repository.get_by_email(parameters.email).await;
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
                None => Err(DbErr::RecordNotUpdated),
            },
            Err(err) => Err(err),
        }
    }

    async fn create_user<'a>(&'a self, user_req: UserCreationDto) -> Option<i32> {
        let active_user = user::ActiveModel {
            name: Set(user_req.name),
            display_name: Set(user_req.display_name),
            email: Set(user_req.email),
            status_id: Set(UserStatuses::Actived as i32),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            password_hash: Set(user_req.password),
            security_stamp: Set(user_req.security_stamp),
            ..Default::default()
        };
        let created = self._user_repository.create(active_user).await;
        match created {
            Err(_) => None,
            Ok(i) => Some(i.id),
        }
    }
}
