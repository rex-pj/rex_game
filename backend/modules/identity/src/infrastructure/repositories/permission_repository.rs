use rex_game_entities::entities::permission::{self, Entity as Permission};
use crate::domain::{
    models::permission_model::PermissionModel,
    repositories::permission_repository_trait::PermissionRepositoryTrait,
};
use chrono::Utc;
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::InfraError;
use sea_orm::{
    sea_query::{Expr, Func},
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, ExprTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct PermissionRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl PermissionRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl PermissionRepositoryTrait for PermissionRepository {
    async fn create(&self, permission_req: PermissionModel) -> Result<i32, InfraError> {
        let db = self._db_connection.as_ref();

        let permission = permission::ActiveModel {
            name: Set(permission_req.name),
            description: Set(permission_req.description),
            code: Set(permission_req.code),
            module: Set(permission_req.module),
            created_by_id: Set(permission_req.created_by_id),
            updated_by_id: Set(permission_req.updated_by_id),
            created_on: Set(Utc::now().fixed_offset()),
            updated_on: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        let inserted = Permission::insert(permission)
            .exec(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()));

        match inserted {
            Ok(updated) => {
                return Ok(updated.last_insert_id);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    async fn get_by_code(&self, code: &str) -> Result<Option<PermissionModel>, InfraError> {
        let db = self._db_connection.as_ref();

        // SeaORM 2.0: Use binary_op for custom SQL expressions
        let code_lower = code.to_lowercase();
        let existing = Permission::find()
            .filter(
                Condition::all()
                    .add(Func::lower(Expr::col(permission::Column::Code)).eq(code_lower))
                    .add(permission::Column::IsActived.eq(true)),
            )
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(Some(self::map_entity_to_model(f))),
            None => Ok(None),
        }
    }

    async fn get_by_codes(&self, codes: Vec<String>) -> Result<Vec<PermissionModel>, InfraError> {
        let db = self._db_connection.as_ref();
        let existing_permissions = Permission::find()
            .filter(permission::Column::Code.is_in(codes))
            .all(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        if existing_permissions.is_empty() {
            return Ok(vec![]);
        }
        let list = existing_permissions
            .into_iter()
            .map(|i| self::map_entity_to_model(i))
            .collect::<Vec<PermissionModel>>();
        Ok(list)
    }

    async fn get_by_name(&self, name: &str) -> Result<Option<PermissionModel>, InfraError> {
        let db = self._db_connection.as_ref();

        // SeaORM 2.0: Use binary_op for custom SQL expressions
        let name_lower = name.to_lowercase();
        let existing = Permission::find()
            .filter(
                Condition::all()
                    .add(Func::lower(Expr::col(permission::Column::Name)).eq(name_lower))
                    .add(permission::Column::IsActived.eq(true)),
            )
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(Some(self::map_entity_to_model(f))),
            None => Ok(None),
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<PermissionModel, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = Permission::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => Ok(self::map_entity_to_model(f)),
            None => Err(InfraError::not_found(
                "Permission not found",
                id.to_string(),
            )),
        }
    }

    async fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListModel<PermissionModel>, InfraError> {
        let db = self._db_connection.as_ref();
        let mut query = Permission::find();

        if let Some(d) = name {
            query = query.filter(permission::Column::Name.eq(d));
        }

        if let Some(n) = description {
            query = query.filter(permission::Column::Description.eq(n));
        }

        query = query
            .columns([
                permission::Column::Id,
                permission::Column::Name,
                permission::Column::Description,
                permission::Column::Code,
                permission::Column::Module,
                permission::Column::CreatedOn,
                permission::Column::UpdatedOn,
                permission::Column::CreatedById,
                permission::Column::UpdatedById,
                permission::Column::IsActived,
            ])
            .order_by(permission::Column::UpdatedOn, sea_orm::Order::Desc);

        match page_size_option {
            Some(page_size) if page > 0 => {
                let paginator = query.paginate(db, page_size);
                let total_count = match paginator.num_items().await {
                    Ok(count) => count,
                    Err(err) => return Err(InfraError::database(err.to_string().as_str())),
                };

                let page_list = paginator.fetch_page(page - 1).await;
                match page_list {
                    Ok(items) => {
                        let list = items
                            .into_iter()
                            .map(|i| self::map_entity_to_model(i))
                            .collect::<Vec<PermissionModel>>();
                        return Ok(PageListModel {
                            items: list,
                            total_count,
                        });
                    }
                    Err(err) => return Err(InfraError::database(err.to_string().as_str())),
                }
            }
            None | Some(_) => {
                let page_list = query.all(db).await;
                match page_list {
                    Ok(items) => {
                        let list = items
                            .into_iter()
                            .map(|i| self::map_entity_to_model(i))
                            .collect::<Vec<PermissionModel>>();
                        return Ok(PageListModel {
                            items: list.clone(),
                            total_count: list.len() as u64,
                        });
                    }
                    Err(err) => return Err(InfraError::database(err.to_string().as_str())),
                }
            }
        }
    }

    async fn update(&self, permission_req: PermissionModel) -> Result<bool, InfraError> {
        let db = self._db_connection.as_ref();

        // Find existing permission
        let permission_option = Permission::find_by_id(permission_req.id)
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        let mut permission: permission::ActiveModel = match permission_option {
            Some(f) => f.into(),
            None => {
                return Err(InfraError::not_found(
                    "Permission not found",
                    permission_req.id.to_string(),
                ))
            }
        };

        permission.updated_by_id = Set(permission_req.updated_by_id);
        permission.description = Set(permission_req.description);
        permission.name = Set(permission_req.name);
        permission.code = Set(permission_req.code);
        permission.module = Set(permission_req.module);
        permission.is_actived = Set(permission_req.is_actived);
        permission.updated_on = Set(Utc::now().fixed_offset());

        match Permission::update(permission).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }
}

fn map_entity_to_model(permission: permission::Model) -> PermissionModel {
    PermissionModel {
        id: permission.id,
        name: permission.name,
        description: permission.description,
        code: permission.code,
        module: permission.module,
        created_on: permission.created_on.with_timezone(&Utc),
        updated_on: permission.updated_on.with_timezone(&Utc),
        created_by_id: permission.created_by_id,
        updated_by_id: permission.updated_by_id,
        is_actived: permission.is_actived,
    }
}
