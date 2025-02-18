use chrono::Utc;
use rex_game_domain::{
    entities::user::{self, Entity as User},
    repositories::user_repository_trait::UserRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, InsertResult, QueryFilter, Set,
    TransactionTrait,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl UserRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl UserRepositoryTrait for UserRepository {
    async fn create(&self, mut user: user::ActiveModel) -> Result<user::Model, DbErr> {
        let db_transaction = self._db_connection.begin().await?;

        user.created_date = Set(Utc::now().fixed_offset());
        user.updated_date = Set(Utc::now().fixed_offset());
        let inserted_user: Result<InsertResult<user::ActiveModel>, DbErr> =
            User::insert(user).exec(&db_transaction).await;

        match inserted_user {
            Ok(inserted) => {
                let updating_user: user::ActiveModel = user::ActiveModel {
                    id: Set(inserted.last_insert_id),
                    created_by_id: Set(Some(inserted.last_insert_id)),
                    updated_by_id: Set(Some(inserted.last_insert_id)),
                    ..Default::default()
                };

                let updated_user = User::update(updating_user).exec(&db_transaction).await;
                match updated_user {
                    Ok(updated) => {
                        db_transaction.commit().await?;
                        return Ok(updated);
                    }
                    Err(err) => {
                        db_transaction.rollback().await?;
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                db_transaction.rollback().await?;
                return Err(err);
            }
        }
    }

    async fn get_by_email(&self, email: String) -> Result<Option<user::Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let flashcard_type = User::find()
            .filter(Condition::all().add(user::Column::Email.eq(email)))
            .one(db)
            .await;

        return flashcard_type;
    }
}
