use crate::entities::{
    permission,
    user_permission::{self, Entity as UserPermission},
};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::user_permission_model::UserPermissionModel,
    repositories::user_permission_repository_trait::UserPermissionRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait, Set,
};
use std::{collections::HashSet, future::Future, pin::Pin, sync::Arc};

#[derive(Clone)]
pub struct UserPermissionRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl UserPermissionRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl UserPermissionRepositoryTrait for UserPermissionRepository {
    async fn create(&self, user_permission_req: UserPermissionModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();
        let user_permission = user_permission::ActiveModel {
            user_id: Set(user_permission_req.user_id),
            permission_id: Set(user_permission_req.permission_id),
            created_by_id: Set(user_permission_req.created_by_id),
            updated_by_id: Set(user_permission_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        match UserPermission::insert(user_permission).exec(db).await {
            Ok(result) => Ok(result.last_insert_id),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }

    fn get_user_permissions(
        &self,
        user_id: i32,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<UserPermissionModel>, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserPermission::find()
                .filter(user_permission::Column::UserId.eq(user_id))
                .find_with_related(permission::Entity)
                .all(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                })?;

            let permissions = existing
                .into_iter()
                .map(|i| {
                    let permission_name = match i.1.first() {
                        Some(permission) => permission.name.to_owned(),
                        None => String::from(""),
                    };

                    let permission_code = match i.1.first() {
                        Some(permission) => permission.code.to_owned(),
                        None => String::from(""),
                    };

                    let permission_module = match i.1.first() {
                        Some(permission) => permission.module.to_owned(),
                        None => String::from(""),
                    };
                    return UserPermissionModel {
                        id: i.0.id,
                        permission_id: i.0.permission_id,
                        user_id: i.0.user_id,
                        permission_name: permission_name,
                        permission_code: permission_code,
                        permission_module: permission_module,
                        created_date: i.0.created_date.with_timezone(&Utc),
                        updated_date: i.0.updated_date.with_timezone(&Utc),
                        created_by_id: i.0.created_by_id,
                        updated_by_id: i.0.updated_by_id,
                        is_actived: i.0.is_actived,
                    };
                })
                .collect::<Vec<UserPermissionModel>>();

            Ok(permissions)
        })
    }

    fn is_user_in_permission(
        &self,
        user_id: i32,
        permission_codes: HashSet<String>,
    ) -> Pin<Box<dyn Future<Output = Result<bool, DomainError>> + Send>> {
        let db_connection = Arc::clone(&self._db_connection);
        let codes = permission_codes.into_iter().collect::<Vec<String>>();

        Box::pin(async move {
            let db = db_connection.as_ref();
            let existing = UserPermission::find()
                .filter(user_permission::Column::UserId.eq(user_id))
                .join(
                    JoinType::InnerJoin,
                    user_permission::Relation::Permission.def(),
                )
                .filter(Condition::all().add(permission::Column::Code.is_in(codes)))
                .one(db)
                .await
                .map_err(|err| {
                    DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
                });

            match existing {
                Ok(permissions) => Ok(permissions.is_some()),
                Err(err) => return Err(err),
            }
        })
    }
}
