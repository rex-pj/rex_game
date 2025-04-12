use chrono::Utc;
use rex_game_domain::{
    entities::role::{self, Entity as Role, Model},
    repositories::role_repository_trait::RoleRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct RoleRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl RoleRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl RoleRepositoryTrait for RoleRepository {
    async fn create(&self, mut role: role::ActiveModel) -> Result<Model, DbErr> {
        let db_transaction = self._db_connection.begin().await?;
        role.created_date = Set(Utc::now().fixed_offset());
        role.updated_date = Set(Utc::now().fixed_offset());
        let inserted = Role::insert(role).exec(&db_transaction).await?;

        let updating_role: role::ActiveModel = role::ActiveModel {
            id: Set(inserted.last_insert_id),
            created_by_id: Set(Some(inserted.last_insert_id)),
            updated_by_id: Set(Some(inserted.last_insert_id)),
            ..Default::default()
        };

        let updated_role = Role::update(updating_role).exec(&db_transaction).await;
        match updated_role {
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

    async fn get_by_name(&self, name: &str) -> Result<Option<Model>, DbErr> {
        let db = self._db_connection.as_ref();
        let existing_role = Role::find()
            .filter(Condition::all().add(role::Column::Name.eq(name)))
            .one(db)
            .await;

        return existing_role;
    }

    async fn get_list(&self, page: u64, page_size: u64) -> Result<(Vec<role::Model>, u64), DbErr> {
        let db = self._db_connection.as_ref();
        let mut query = Role::find();

        query = query.order_by(role::Column::UpdatedDate, sea_orm::Order::Desc);

        let paginator = query.paginate(db, page_size);

        let num_pages = paginator.num_pages().await?;
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
