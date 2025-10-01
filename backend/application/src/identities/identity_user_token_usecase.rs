use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    identities::{
        identity_user_token_usecase_trait::IdentityUserTokenUseCaseTrait,
        user_token_creation_dto::UserTokenCreationDto, user_token_dto::UserTokenDto,
        user_token_updation_dto::UserTokenUpdationDto,
    },
};
use rex_game_domain::{
    errors::domain_error::ErrorType, models::user_token_model::UserTokenModel,
    repositories::user_token_repository_trait::UserTokenRepositoryTrait,
};

#[derive(Clone)]
pub struct IdentityUserTokenUseCase<UTR>
where
    UTR: UserTokenRepositoryTrait,
{
    _user_token_repository: UTR,
}

impl<UTR> IdentityUserTokenUseCase<UTR>
where
    UTR: UserTokenRepositoryTrait,
{
    pub fn new(user_token_repository: UTR) -> Self {
        Self {
            _user_token_repository: user_token_repository,
        }
    }
}

impl<UTR> IdentityUserTokenUseCaseTrait for IdentityUserTokenUseCase<UTR>
where
    UTR: UserTokenRepositoryTrait,
{
    async fn create_user_token(
        &self,
        user_token_req: UserTokenCreationDto,
    ) -> Result<i32, ApplicationError> {
        let user_token_model = UserTokenModel {
            token: user_token_req.token,
            user_id: user_token_req.user_id,
            created_by_id: user_token_req.created_by_id,
            updated_by_id: user_token_req.updated_by_id,
            expiration: user_token_req.expiration,
            purpose: user_token_req.purpose,
            is_actived: true, // Default to true for new tokens
            ..Default::default()
        };
        let result = self._user_token_repository.create(user_token_model).await;
        match result {
            Ok(id) => Ok(id),
            Err(err) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                &err.message,
                err.details,
            )),
        }
    }

    async fn get_user_token_by_token(&self, token: &str) -> Result<UserTokenDto, ApplicationError> {
        let existing = self._user_token_repository.get_by_token(token).await;
        match existing {
            Ok(f) => Ok(UserTokenDto {
                id: f.id,
                user_id: f.user_id,
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                created_date: f.created_date,
                updated_date: f.updated_date,
                token: f.token,
                expiration: f.expiration,
                is_actived: f.is_actived,
                purpose: f.purpose,
            }),
            Err(err) => match err.kind {
                ErrorType::DatabaseError => Err(ApplicationError::new(
                    ApplicationErrorKind::DatabaseError,
                    &err.message,
                    err.details,
                )),
                ErrorType::NotFound => Err(ApplicationError::new(
                    ApplicationErrorKind::NotFound,
                    &err.message,
                    err.details,
                )),
                ErrorType::EmailError => Err(ApplicationError::new(
                    ApplicationErrorKind::InternalServerError,
                    &err.message,
                    err.details,
                )),
            },
        }
    }

    async fn update_user_token<'a>(
        &'a self,
        id: i32,
        user_token_req: UserTokenUpdationDto,
    ) -> Result<bool, ApplicationError> {
        let mut existing = self
            ._user_token_repository
            .get_by_id(id)
            .await
            .map_err(|err| {
                ApplicationError::new(ApplicationErrorKind::NotFound, &err.message, err.details)
            })?;

        match user_token_req.is_actived {
            Some(is_actived) => existing.is_actived = is_actived,
            None => {}
        };
        existing.updated_by_id = user_token_req.updated_by_id;
        let updated = self._user_token_repository.update(existing).await;
        match updated {
            Ok(i) => Ok(i),
            Err(err) => Err(ApplicationError::new(
                ApplicationErrorKind::InternalServerError,
                &err.message,
                err.details,
            )),
        }
    }
}
