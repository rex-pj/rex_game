use crate::{
    entities::{
        role,
        user::{self, Entity as User},
        user_role,
    },
    transaction_manager::SeaOrmTransactionWrapper,
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{page_list_model::PageListModel, user_model::UserModel, user_statuses::UserStatuses},
    repositories::user_repository_trait::UserRepositoryTrait,
    transaction_manager_trait::TransactionWrapperTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
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
    async fn get_paged_list(
        &self,
        display_name: Option<String>,
        name: Option<String>,
        email: Option<String>,
        role_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> Result<PageListModel<UserModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let mut query =
            User::find().filter(user::Column::StatusId.ne(UserStatuses::Deleted as i32));
        if let Some(role) = role_name {
            query = query
                .join(JoinType::InnerJoin, user_role::Relation::Role.def())
                .filter(role::Column::Name.eq(role));
        }

        if let Some(d) = display_name {
            query = query.filter(user::Column::DisplayName.eq(d));
        }

        if let Some(n) = name {
            query = query.filter(user::Column::Name.eq(n));
        }

        if let Some(mail) = email {
            query = query.filter(user::Column::Email.eq(mail));
        }

        query = query
            .order_by(user::Column::UpdatedDate, sea_orm::Order::Desc)
            .distinct();

        let paginator = query.paginate(db, page_size);
        let total_count = paginator.num_items().await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        let page_list = paginator.fetch_page(page - 1).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        let items = page_list
            .into_iter()
            .map(|i| UserModel {
                id: i.id,
                name: i.name,
                display_name: i.display_name,
                email: i.email,
                status_id: i.status_id,
                created_date: i.created_date.with_timezone(&Utc),
                updated_date: i.updated_date.with_timezone(&Utc),
                created_by_id: i.created_by_id,
                updated_by_id: i.updated_by_id,
                ..Default::default()
            })
            .collect::<Vec<UserModel>>();
        return Ok(PageListModel { items, total_count });
    }

    async fn create(&self, user_req: UserModel) -> Result<i32, DomainError> {
        let db_transaction = match self._db_connection.begin().await {
            Ok(transaction) => transaction,
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ))
            }
        };

        let new_user = user::ActiveModel {
            display_name: Set(user_req.display_name),
            email: Set(user_req.email),
            name: Set(user_req.name),
            password_hash: Set(user_req.password_hash),
            security_stamp: Set(user_req.security_stamp),
            status_id: Set(user_req.status_id),
            created_by_id: Set(user_req.created_by_id),
            updated_by_id: Set(user_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };

        let inserted_user = User::insert(new_user)
            .exec(&db_transaction)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
            ..Default::default()
        };

        let updated_user = User::update(updating_user).exec(&db_transaction).await;
        match updated_user {
            Ok(_) => {
                db_transaction.commit().await.map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;
                return Ok(inserted_user.last_insert_id);
            }
            Err(err) => {
                db_transaction.rollback().await.map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ));
            }
        }
    }

    async fn create_without_commit(
        &self,
        user_req: UserModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<i32, DomainError> {
        let new_user = user::ActiveModel {
            display_name: Set(user_req.display_name),
            email: Set(user_req.email),
            name: Set(user_req.name),
            password_hash: Set(user_req.password_hash),
            security_stamp: Set(user_req.security_stamp),
            status_id: Set(user_req.status_id),
            created_by_id: Set(user_req.created_by_id),
            updated_by_id: Set(user_req.updated_by_id),
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
        let inserted_user = User::insert(new_user)
            .exec(&transact.transaction)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
            ..Default::default()
        };

        let updated_user = User::update(updating_user)
            .exec(&transact.transaction)
            .await;
        match updated_user {
            Ok(updated) => {
                return Ok(updated.id);
            }
            Err(err) => {
                return Err(DomainError::new(
                    ErrorType::DatabaseError,
                    err.to_string().as_str(),
                    None,
                ));
            }
        }
    }

    async fn get_by_email(&self, email: String) -> Result<UserModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = User::find()
            .filter(Condition::all().add(user::Column::Email.eq(email)))
            .one(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;

        match existing {
            Some(f) => Ok(UserModel {
                id: f.id,
                name: f.name,
                display_name: f.display_name,
                email: f.email,
                password_hash: f.password_hash,
                security_stamp: f.security_stamp,
                status_id: f.status_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
            }),
            None => Err(DomainError::new(
                ErrorType::NotFound,
                "User not found",
                None,
            )),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<UserModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = User::find_by_id(id).one(db).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        match existing {
            Some(f) => Ok(UserModel {
                id: f.id,
                name: f.name,
                display_name: f.display_name,
                email: f.email,
                password_hash: f.password_hash,
                security_stamp: f.security_stamp,
                status_id: f.status_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
            }),
            None => Err(DomainError::new(
                ErrorType::NotFound,
                "User not found",
                None,
            )),
        }
    }

    async fn update(&self, user_req: UserModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = User::find_by_id(user_req.id).one(db).await;
        let user_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut existing_user: user::ActiveModel = match user_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        existing_user.updated_by_id = Set(user_req.updated_by_id);
        existing_user.updated_date = Set(Utc::now().fixed_offset());
        existing_user.display_name = Set(user_req.display_name);
        existing_user.email = Set(user_req.email);
        existing_user.name = Set(user_req.name);
        existing_user.status_id = Set(user_req.status_id);

        match User::update(existing_user).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}
