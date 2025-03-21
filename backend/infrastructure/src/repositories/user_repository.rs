use chrono::Utc;
use rex_game_domain::{
    entities::user::{self, Entity as User},
    repositories::user_repository_trait::UserRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, Set, TransactionTrait,
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
        let inserted_user = User::insert(user).exec(&db_transaction).await?; //.exec(&db_transaction).await?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
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

    async fn create_without_commit(
        &self,
        mut user: user::ActiveModel,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> Result<user::Model, DbErr> {
        let db_transaction = match database_transaction {
            Some(transaction) => transaction,
            None => return Err(DbErr::RecordNotInserted),
        };

        user.created_date = Set(Utc::now().fixed_offset());
        user.updated_date = Set(Utc::now().fixed_offset());
        let inserted_user = User::insert(user).exec(db_transaction).await?; //.exec(&db_transaction).await?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
            ..Default::default()
        };

        let updated_user = User::update(updating_user).exec(db_transaction).await;
        updated_user
    }

    async fn get_by_email(&self, email: String) -> Result<Option<user::Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let existing_user = User::find()
            .filter(Condition::all().add(user::Column::Email.eq(email)))
            .one(db)
            .await;

        return existing_user;
    }
}
