use crate::domain::{
    models::{user_model::UserModel, user_statuses::UserStatuses},
    repositories::user_repository_trait::UserRepositoryTrait,
};
use rex_game_entities::entities::{
    role,
    user::{self, Entity as User},
    user_role,
};
use chrono::Utc;
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::domain::transaction_manager_trait::TransactionWrapperTrait;
use rex_game_shared::infrastructure::database::SeaOrmTransactionWrapper;
use rex_game_shared::InfraError;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Set, TransactionTrait,
};
use std::{future::Future, pin::Pin, sync::Arc};

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
    ) -> Result<PageListModel<UserModel>, InfraError> {
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
            .order_by(user::Column::UpdatedOn, sea_orm::Order::Desc)
            .distinct();

        let paginator = query.paginate(db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        let page_list = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        let items = page_list
            .into_iter()
            .map(|i| UserModel {
                id: i.id,
                name: i.name,
                display_name: i.display_name,
                email: i.email,
                status_id: i.status_id,
                created_on: i.created_on.with_timezone(&Utc),
                updated_on: i.updated_on.with_timezone(&Utc),
                created_by_id: i.created_by_id,
                updated_by_id: i.updated_by_id,
                ..Default::default()
            })
            .collect::<Vec<UserModel>>();
        return Ok(PageListModel { items, total_count });
    }

    async fn create(&self, user_req: UserModel) -> Result<i32, InfraError> {
        let db_transaction = match self._db_connection.begin().await {
            Ok(transaction) => transaction,
            Err(err) => return Err(InfraError::database(err.to_string().as_str())),
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
            created_on: Set(Utc::now().fixed_offset()),
            updated_on: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };

        let inserted_user = User::insert(new_user)
            .exec(&db_transaction)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
            ..Default::default()
        };

        let updated_user = User::update(updating_user).exec(&db_transaction).await;
        match updated_user {
            Ok(_) => {
                db_transaction
                    .commit()
                    .await
                    .map_err(|err| InfraError::database(err.to_string().as_str()))?;
                return Ok(inserted_user.last_insert_id);
            }
            Err(err) => {
                db_transaction
                    .rollback()
                    .await
                    .map_err(|err| InfraError::database(err.to_string().as_str()))?;
                return Err(InfraError::database(err.to_string().as_str()));
            }
        }
    }

    async fn create_without_commit(
        &self,
        user_req: UserModel,
        transaction: Box<&dyn TransactionWrapperTrait>,
    ) -> Result<i32, InfraError> {
        let new_user = user::ActiveModel {
            display_name: Set(user_req.display_name),
            email: Set(user_req.email),
            name: Set(user_req.name),
            password_hash: Set(user_req.password_hash),
            security_stamp: Set(user_req.security_stamp),
            status_id: Set(user_req.status_id),
            created_by_id: Set(user_req.created_by_id),
            updated_by_id: Set(user_req.updated_by_id),
            created_on: Set(Utc::now().fixed_offset()),
            updated_on: Set(Utc::now().fixed_offset()),
            ..Default::default()
        };

        let it = transaction.as_ref().as_any();
        let transact = match it.downcast_ref::<SeaOrmTransactionWrapper>() {
            Some(i) => i,
            None => return Err(InfraError::database("Unable to cast the transaction")),
        };
        let inserted_user = User::insert(new_user)
            .exec(transact.txn.as_ref().unwrap())
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;
        let updating_user: user::ActiveModel = user::ActiveModel {
            id: Set(inserted_user.last_insert_id),
            created_by_id: Set(Some(inserted_user.last_insert_id)),
            updated_by_id: Set(Some(inserted_user.last_insert_id)),
            ..Default::default()
        };

        let updated_user = User::update(updating_user)
            .exec(transact.txn.as_ref().unwrap())
            .await;
        match updated_user {
            Ok(updated) => {
                return Ok(updated.id);
            }
            Err(err) => {
                return Err(InfraError::database(err.to_string().as_str()));
            }
        }
    }

    fn get_by_email(
        &self,
        email: &str,
    ) -> Pin<Box<dyn Future<Output = Result<UserModel, InfraError>> + Send>> {
        let db = self._db_connection.clone();
        let email = email.to_owned();
        Box::pin(async move {
            let existing = User::find()
                .filter(Condition::all().add(user::Column::Email.eq(email.to_owned())))
                .one(db.as_ref())
                .await
                .map_err(|err| InfraError::database(err.to_string().as_str()))?;

            match existing {
                Some(f) => Ok(self::map_entity_to_model(f)),
                None => Err(InfraError::not_found("User not found", email.to_string())),
            }
        })
    }

    async fn get_by_name(&self, name: &String) -> Result<UserModel, InfraError> {
        let db = self._db_connection.clone();
        let existing = User::find()
            .filter(Condition::all().add(user::Column::Name.eq(name.to_owned())))
            .one(db.as_ref())
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(self::map_entity_to_model(f)),
            None => Err(InfraError::not_found("User not found", name)),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<UserModel, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = User::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(self::map_entity_to_model(f)),
            None => Err(InfraError::not_found("User not found", id.to_string())),
        }
    }

    async fn update(&self, user_req: UserModel) -> Result<bool, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = User::find_by_id(user_req.id).one(db).await;
        let user_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut existing_user: user::ActiveModel = match user_option {
            Some(f) => f.into(),
            None => {
                return Err(InfraError::not_found(
                    "User not found",
                    user_req.id.to_string(),
                ))
            }
        };

        existing_user.updated_by_id = Set(user_req.updated_by_id);
        existing_user.updated_on = Set(Utc::now().fixed_offset());
        existing_user.display_name = Set(user_req.display_name);
        existing_user.email = Set(user_req.email);
        existing_user.name = Set(user_req.name);
        existing_user.status_id = Set(user_req.status_id);
        existing_user.password_hash = Set(user_req.password_hash);

        match User::update(existing_user).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }
}

fn map_entity_to_model(f: user::Model) -> UserModel {
    UserModel {
        id: f.id,
        name: f.name,
        display_name: f.display_name,
        email: f.email,
        password_hash: f.password_hash,
        security_stamp: f.security_stamp,
        status_id: f.status_id,
        created_on: f.created_on.with_timezone(&Utc),
        updated_on: f.updated_on.with_timezone(&Utc),
        created_by_id: f.created_by_id,
        updated_by_id: f.updated_by_id,
    }
}
