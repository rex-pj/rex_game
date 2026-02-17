use rex_game_entities::entities::role::{self, Entity as Role};
use crate::domain::{
    models::role_model::RoleModel, repositories::role_repository_trait::RoleRepositoryTrait,
};
use chrono::Utc;
use rex_game_shared::domain::models::page_list_model::PageListModel;
use rex_game_shared::InfraError;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
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
    async fn create(&self, role_req: RoleModel) -> Result<i32, InfraError> {
        let db = self._db_connection.as_ref();

        let role = role::ActiveModel {
            name: Set(role_req.name),
            description: Set(role_req.description),
            created_by_id: Set(role_req.created_by_id),
            updated_by_id: Set(role_req.updated_by_id),
            created_on: Set(Utc::now().fixed_offset()),
            updated_on: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            ..Default::default()
        };

        let inserted = Role::insert(role)
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

    async fn get_by_name(&self, name: &str) -> Option<RoleModel> {
        let db = self._db_connection.as_ref();
        let existing = Role::find()
            .filter(
                Condition::all()
                    .add(role::Column::Name.eq(name))
                    .add(role::Column::IsActived.eq(true)),
            )
            .one(db)
            .await;

        match existing {
            Ok(f) => match f {
                Some(role) => Some(self::map_entity_to_model(role)),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<RoleModel, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = Role::find_by_id(id)
            .one(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        match existing {
            Some(f) => {
                if f.is_actived {
                    return Ok(self::map_entity_to_model(f));
                }

                Err(InfraError::not_found("Role not found", id.to_string()))
            }
            None => Err(InfraError::not_found("Role not found", id.to_string())),
        }
    }

    async fn get_by_ids(&self, ids: Vec<i32>) -> Result<Vec<RoleModel>, InfraError> {
        let db = self._db_connection.as_ref();
        let existing_roles = Role::find()
            .filter(role::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(|err| InfraError::database(err.to_string().as_str()))?;

        let list = existing_roles
            .into_iter()
            .map(|i| self::map_entity_to_model(i))
            .collect::<Vec<RoleModel>>();
        return Ok(list);
    }

    async fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListModel<RoleModel>, InfraError> {
        let db = self._db_connection.as_ref();
        let mut query = Role::find().filter(role::Column::IsActived.eq(true));

        if let Some(d) = name {
            query = query.filter(role::Column::Name.eq(d));
        }

        if let Some(n) = description {
            query = query.filter(role::Column::Description.eq(n));
        }

        query = query.order_by(role::Column::UpdatedOn, sea_orm::Order::Desc);

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
                            .collect::<Vec<RoleModel>>();
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
                            .collect::<Vec<RoleModel>>();
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

    async fn update(&self, role_req: RoleModel) -> Result<bool, InfraError> {
        let db = self._db_connection.as_ref();
        let existing = Role::find_by_id(role_req.id).one(db).await;
        let role_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut role: role::ActiveModel = match role_option {
            Some(f) => f.into(),
            None => {
                return Err(InfraError::not_found(
                    "Role not found",
                    role_req.id.to_string(),
                ))
            }
        };

        role.updated_by_id = Set(role_req.updated_by_id);
        role.description = Set(role_req.description);
        role.is_actived = Set(role_req.is_actived);
        role.name = Set(role_req.name);
        role.updated_on = Set(Utc::now().fixed_offset());

        match Role::update(role).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(InfraError::database(err.to_string().as_str())),
        }
    }
}

fn map_entity_to_model(role: role::Model) -> RoleModel {
    RoleModel {
        id: role.id,
        name: role.name,
        description: role.description,
        created_on: role.created_on.with_timezone(&Utc),
        updated_on: role.updated_on.with_timezone(&Utc),
        created_by_id: role.created_by_id,
        updated_by_id: role.updated_by_id,
        is_actived: role.is_actived,
    }
}
