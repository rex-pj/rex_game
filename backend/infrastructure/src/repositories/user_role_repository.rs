use chrono::Utc;
use rex_game_domain::{
    entities::{
        role,
        user_role::{self, Entity as UserRole},
    },
    repositories::user_role_repository_trait::UserRoleRepositoryTrait,
};
use sea_orm::{
    sea_query::QueryStatementWriter, sqlx::query_builder, ColumnTrait, Condition,
    DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, InsertResult, JoinType,
    QueryFilter, QuerySelect, QueryTrait, RelationTrait, Set,
};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};

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

    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<user_role::Model>, DbErr>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let roles = UserRole::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, role::Relation::UserRole.def())
                .all(db)
                .await?
                .into_iter()
                .map(|role| role)
                .collect::<Vec<user_role::Model>>();

            Ok(roles)
        })
    }

    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DbErr>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        let role_names = roles.into_iter().collect::<Vec<String>>();

        Box::pin(async move {
            let db = db_connection.as_ref();
            let is_ok = UserRole::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, user_role::Relation::Role.def())
                .filter(Condition::all().add(role::Column::Name.is_in(role_names)))
                .one(db)
                .await?
                .is_some();

            Ok(is_ok)
        })
    }
}
