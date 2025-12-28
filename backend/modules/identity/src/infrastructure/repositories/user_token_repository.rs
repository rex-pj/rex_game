use rex_game_entities::entities::user_token::{self, Entity as UserToken};
use crate::domain::{
    models::user_token_model::UserTokenModel,
    repositories::user_token_repository_trait::UserTokenRepositoryTrait,
};
use chrono::Utc;
use rex_game_shared::InfraError;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserTokenRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl UserTokenRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl UserTokenRepositoryTrait for UserTokenRepository {
    async fn create(&self, user_token_req: UserTokenModel) -> Result<i32, InfraError> {
        let db = self._db_connection.as_ref();
        let user_token = user_token::ActiveModel {
            user_id: Set(user_token_req.user_id),
            created_by_id: Set(user_token_req.created_by_id),
            updated_by_id: Set(user_token_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            token: Set(user_token_req.token),
            expiration: Set(user_token_req.expiration),
            is_actived: Set(user_token_req.is_actived),
            purpose: Set(user_token_req.purpose),
            ..Default::default()
        };

        let inserted = UserToken::insert(user_token)
            .exec(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()));

        match inserted {
            Ok(updated) => {
                return Ok(updated.last_insert_id);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<UserTokenModel, InfraError> {
        let db = self._db_connection.clone();
        let existing = UserToken::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(self::map_entity_to_model(f)),
            None => Err(InfraError::not_found(
                "User Token not found",
                id.to_string(),
            )),
        }
    }

    async fn get_by_token(&self, token: &str) -> Result<UserTokenModel, InfraError> {
        let db = self._db_connection.clone();
        let token = token.to_owned();
        let existing = UserToken::find()
            .filter(Condition::all().add(user_token::Column::Token.eq(token)))
            .one(db.as_ref())
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(self::map_entity_to_model(f)),
            None => Err(InfraError::not_found(
                "User Token not found",
                "".to_string(),
            )),
        }
    }

    async fn update(&self, user_token_req: UserTokenModel) -> Result<bool, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = UserToken::find_by_id(user_token_req.id).one(db).await;
        let user_token_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut existing_user_token: user_token::ActiveModel = match user_token_option {
            Some(f) => f.into(),
            None => {
                return Err(InfraError::not_found(
                    "User Token not found",
                    user_token_req.id.to_string(),
                ))
            }
        };

        existing_user_token.updated_by_id = Set(user_token_req.updated_by_id);
        existing_user_token.updated_date = Set(Utc::now().fixed_offset());
        existing_user_token.is_actived = Set(user_token_req.is_actived);

        match UserToken::update(existing_user_token).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }
}

fn map_entity_to_model(f: user_token::Model) -> UserTokenModel {
    UserTokenModel {
        id: f.id,
        user_id: f.user_id,
        created_by_id: f.created_by_id,
        updated_by_id: f.updated_by_id,
        created_date: f.created_date.with_timezone(&Utc),
        updated_date: f.updated_date.with_timezone(&Utc),
        token: f.token,
        expiration: f.expiration,
        is_actived: f.is_actived,
        purpose: f.purpose,
        ..Default::default()
    }
}
