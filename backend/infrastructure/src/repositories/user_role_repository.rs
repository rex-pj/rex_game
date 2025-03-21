use chrono::Utc;
use rex_game_domain::{
    entities::user_role::{self, Entity as UserRole},
    repositories::user_role_repository_trait::UserRoleRepositoryTrait,
};
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, InsertResult, Set};
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
    async fn create_without_commit(
        &self,
        mut user_role: user_role::ActiveModel,
        database_transaction: Option<&DatabaseTransaction>,
    ) -> Result<InsertResult<user_role::ActiveModel>, DbErr> {
        let db_transaction = match database_transaction {
            Some(transaction) => transaction,
            None => return Err(DbErr::RecordNotInserted),
        };

        user_role.created_date = Set(Utc::now().fixed_offset());
        user_role.updated_date = Set(Utc::now().fixed_offset());
        let inserted_user_role = UserRole::insert(user_role).exec(db_transaction).await;
        inserted_user_role
    }
}
