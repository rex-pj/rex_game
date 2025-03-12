use chrono::Utc;
use rex_game_domain::{
    entities::user_role::{self, Entity as UserRole},
    repositories::user_role_repository_trait::UserRoleRepositoryTrait,
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, InsertResult, Set, TransactionTrait};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRoleRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl UserRoleRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl UserRoleRepositoryTrait for UserRoleRepository {
    async fn create(
        &self,
        mut user_role: user_role::ActiveModel,
    ) -> Result<InsertResult<user_role::ActiveModel>, DbErr> {
        let db_transaction = self._db_connection.begin().await?;

        user_role.created_date = Set(Utc::now().fixed_offset());
        user_role.updated_date = Set(Utc::now().fixed_offset());
        let inserted = UserRole::insert(user_role).exec(&db_transaction).await;

        return inserted;
    }
}
