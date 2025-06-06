use crate::{
    entities::{
        role,
        user_role::{self, Entity as UserRole},
    },
    transaction_manager::SeaOrmTransactionWrapper,
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::user_role_model::UserRoleModel,
    repositories::user_role_repository_trait::UserRoleRepositoryTrait,
    transaction_manager_trait::TransactionWrapperTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait, Set,
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

    fn get_user_roles(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserRoleModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserRole::find()
                .filter(user_role::Column::UserId.eq(user_id))
                .join(JoinType::InnerJoin, role::Relation::UserRole.def())
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let roles = existing
                .into_iter()
                .map(|i| UserRoleModel {
                    id: i.id,
                    role_id: i.role_id,
                    user_id: i.user_id,
                    created_date: i.created_date.with_timezone(&Utc),
                    updated_date: i.updated_date.with_timezone(&Utc),
                    created_by_id: i.created_by_id,
                    updated_by_id: i.updated_by_id,
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
