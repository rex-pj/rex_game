use crate::{
    entities::{
        role, user,
        user_role::{self, Entity as UserRole},
    },
    transaction_manager::SeaOrmTransactionWrapper,
};
use chrono::{DateTime, Utc};
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::user_role_model::UserRoleModel,
    repositories::user_role_repository_trait::UserRoleRepositoryTrait,
    transaction_manager_trait::TransactionWrapperTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, FromQueryResult, JoinType,
    QueryFilter, QuerySelect, RelationTrait, Set,
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

#[derive(FromQueryResult)]
struct UserRoleWithUser {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub role_id: i32,
    pub role_name: String,
    pub created_by_id: i32,
    pub created_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub updated_by_id: i32,
    pub is_actived: bool,
}

impl UserRoleRepositoryTrait for UserRoleRepository {
    async fn create_without_commit(
        &self,
        user_role_req: UserRoleModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<i32, DomainError> {
        let user_role = user_role::ActiveModel {
            user_id: Set(user_role_req.user_id),
            role_id: Set(user_role_req.role_id),
            created_by_id: Set(user_role_req.created_by_id),
            updated_by_id: Set(user_role_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        let it = transaction.as_ref().as_any();
        let transact = match it.downcast_ref::<SeaOrmTransactionWrapper>() {
            Some(i) => i,
            None => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    "Unable to cast the transaction",
                    None,
                ))
            }
        };
        match UserRole::insert(user_role)
            .exec(&transact.transaction)
            .await
        {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn create_many(&self, user_role_req: Vec<UserRoleModel>) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let user_roles = user_role_req
            .into_iter()
            .map(|req| user_role::ActiveModel {
                user_id: Set(req.user_id),
                role_id: Set(req.role_id),
                created_by_id: Set(req.created_by_id),
                updated_by_id: Set(req.updated_by_id),
                created_date: Set(Utc::now().fixed_offset()),
                updated_date: Set(Utc::now().fixed_offset()),
                is_actived: Set(true),
                ..Default::default()
            })
            .collect::<Vec<user_role::ActiveModel>>();
        match UserRole::insert_many(user_roles).exec(db).await {
            Ok(result) => match result.last_insert_id {
                Some(id) => Ok(id),
                None => Ok(0), // insert_many may return None if empty
            },
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    async fn delete_many(
        &self,
        user_id: i32,
        user_role_req: Vec<UserRoleModel>,
    ) -> Result<u64, DomainError> {
        let db = self._db_connection.as_ref();

        let delete_role_ids = user_role_req
            .into_iter()
            .map(|f| f.role_id)
            .collect::<Vec<i32>>();
        match UserRole::delete_many()
            .filter(
                Condition::all()
                    .add(user_role::Column::RoleId.is_in(delete_role_ids))
                    .add(user_role::Column::UserId.eq(user_id)),
            )
            .exec(db)
            .await
        {
            Ok(result) => Ok(result.rows_affected),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    fn get_user_roles_by_user_id(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserRole::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, user_role::Relation::User1.def())
                .join(JoinType::InnerJoin, user_role::Relation::Role.def())
                .select_only()
                .column_as(user::Column::Name, "user_name")
                .column_as(role::Column::Name, "role_name")
                .columns([
                    user_role::Column::Id,
                    user_role::Column::RoleId,
                    user_role::Column::CreatedDate,
                    user_role::Column::CreatedById,
                    user_role::Column::UpdatedDate,
                    user_role::Column::UpdatedById,
                    user_role::Column::IsActived,
                    user_role::Column::UserId,
                ])
                .into_model::<UserRoleWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let roles = existing
                .into_iter()
                .map(|i| {
                    return UserRoleModel {
                        id: i.id,
                        role_id: i.role_id,
                        user_id: i.user_id,
                        user_name: i.user_name,
                        role_name: i.role_name,
                        created_date: i.created_date.with_timezone(&Utc),
                        updated_date: i.updated_date.with_timezone(&Utc),
                        created_by_id: i.created_by_id,
                        updated_by_id: i.updated_by_id,
                        is_actived: i.is_actived,
                    };
                })
                .collect::<Vec<UserRoleModel>>();

            Ok(roles)
        })
    }

    fn get_list(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserRole::find()
                .join(JoinType::InnerJoin, user_role::Relation::User1.def())
                .join(JoinType::InnerJoin, user_role::Relation::Role.def())
                .select_only()
                .column_as(user::Column::Name, "user_name")
                .column_as(role::Column::Name, "role_name")
                .columns([
                    user_role::Column::Id,
                    user_role::Column::RoleId,
                    user_role::Column::CreatedDate,
                    user_role::Column::CreatedById,
                    user_role::Column::UpdatedDate,
                    user_role::Column::UpdatedById,
                    user_role::Column::IsActived,
                    user_role::Column::UserId,
                ])
                .into_model::<UserRoleWithUser>()
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let roles = existing
                .into_iter()
                .map(|i| {
                    return UserRoleModel {
                        id: i.id,
                        role_id: i.role_id,
                        user_id: i.user_id,
                        user_name: i.user_name,
                        role_name: i.role_name,
                        created_date: i.created_date.with_timezone(&Utc),
                        updated_date: i.updated_date.with_timezone(&Utc),
                        created_by_id: i.created_by_id,
                        updated_by_id: i.updated_by_id,
                        is_actived: i.is_actived,
                    };
                })
                .collect::<Vec<UserRoleModel>>();

            Ok(roles)
        })
    }

    fn is_user_in_role(
        &self,
        user_id: i32,
        roles: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        let role_names = roles.into_iter().collect::<Vec<String>>();

        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserRole::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, user_role::Relation::Role.def())
                .filter(Condition::all().add(role::Column::Name.is_in(role_names)))
                .one(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                });

            match existing {
                Ok(roles) => Ok(roles.is_some()),
                Err(err) => return Err(err),
            }
        })
    }
}
