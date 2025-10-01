use crate::entities::mail_template::{self, Entity as MailTemplate};
use chrono::Utc;
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    models::{mail_template_model::MailTemplateModel, page_list_model::PageListModel},
    repositories::mail_template_repository_trait::MailTemplateRepositoryTrait,
};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct MailTemplateRepository {
    _db_connection: Arc<DatabaseConnection>,
}

impl MailTemplateRepository {
    pub fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        Self {
            _db_connection: db_connection,
        }
    }
}

impl MailTemplateRepositoryTrait for MailTemplateRepository {
    async fn create(&self, mail_template_req: MailTemplateModel) -> Result<i32, DomainError> {
        let db = self._db_connection.as_ref();

        let mail_template = mail_template::ActiveModel {
            name: Set(mail_template_req.name),
            subject: Set(mail_template_req.subject),
            body: Set(mail_template_req.body),
            created_by_id: Set(mail_template_req.created_by_id),
            updated_by_id: Set(mail_template_req.updated_by_id),
            created_date: Set(Utc::now().fixed_offset()),
            updated_date: Set(Utc::now().fixed_offset()),
            is_actived: Set(true),
            is_enabled: Set(true),
            ..Default::default()
        };

        let inserted = MailTemplate::insert(mail_template)
            .exec(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            });

        match inserted {
            Ok(updated) => {
                return Ok(updated.last_insert_id);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    async fn get_by_name(&self, name: &str) -> Option<MailTemplateModel> {
        let db = self._db_connection.as_ref();
        let existing = MailTemplate::find()
            .filter(
                Condition::all()
                    .add(mail_template::Column::Name.eq(name))
                    .add(mail_template::Column::IsActived.eq(true)),
            )
            .one(db)
            .await;

        match existing {
            Ok(f) => match f {
                Some(mail_template) => Some(self::map_entity_to_model(mail_template)),
                None => None,
            },
            Err(_) => None,
        }
    }

    async fn get_by_id(&self, id: i32) -> Result<MailTemplateModel, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = MailTemplate::find_by_id(id).one(db).await.map_err(|err| {
            DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
        })?;

        if let Some(f) = existing {
            if f.is_actived {
                return Ok(self::map_entity_to_model(f));
            }

            return Err(DomainError::new(
                ErrorType::NotFound,
                "MailTemplate not found",
                None,
            ));
        }

        Err(DomainError::new(
            ErrorType::NotFound,
            "MailTemplate not found",
            None,
        ))
    }

    async fn get_by_ids(&self, ids: Vec<i32>) -> Result<Vec<MailTemplateModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let existing_mail_templates = MailTemplate::find()
            .filter(mail_template::Column::Id.is_in(ids))
            .all(db)
            .await
            .map_err(|err| {
                DomainError::new(ErrorType::DatabaseError, err.to_string().as_str(), None)
            })?;

        let list = existing_mail_templates
            .into_iter()
            .map(|i| self::map_entity_to_model(i))
            .collect::<Vec<MailTemplateModel>>();
        return Ok(list);
    }

    async fn get_paged_list(
        &self,
        name: Option<String>,
        subject: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListModel<MailTemplateModel>, DomainError> {
        let db = self._db_connection.as_ref();
        let mut query = MailTemplate::find().filter(mail_template::Column::IsActived.eq(true));

        if let Some(d) = name {
            query = query.filter(mail_template::Column::Name.eq(d));
        }

        if let Some(n) = subject {
            query = query.filter(mail_template::Column::Subject.eq(n));
        }

        query = query.order_by(mail_template::Column::UpdatedDate, sea_orm::Order::Desc);

        match page_size_option {
            Some(page_size) if page > 0 => {
                let paginator = query.paginate(db, page_size);
                let total_count = match paginator.num_items().await {
                    Ok(count) => count,
                    Err(err) => {
                        return Err(DomainError::new(
                            ErrorType::DatabaseError,
                            err.to_string().as_str(),
                            None,
                        ))
                    }
                };

                let page_list = paginator.fetch_page(page - 1).await;
                match page_list {
                    Ok(items) => {
                        let list = items
                            .into_iter()
                            .map(|i| self::map_entity_to_model(i))
                            .collect::<Vec<MailTemplateModel>>();
                        return Ok(PageListModel {
                            items: list,
                            total_count,
                        });
                    }
                    Err(err) => {
                        return Err(DomainError::new(
                            ErrorType::DatabaseError,
                            err.to_string().as_str(),
                            None,
                        ))
                    }
                }
            }
            None | Some(_) => {
                let page_list = query.all(db).await;
                match page_list {
                    Ok(items) => {
                        let list = items
                            .into_iter()
                            .map(|i| self::map_entity_to_model(i))
                            .collect::<Vec<MailTemplateModel>>();
                        return Ok(PageListModel {
                            items: list.clone(),
                            total_count: list.len() as u64,
                        });
                    }
                    Err(err) => {
                        return Err(DomainError::new(
                            ErrorType::DatabaseError,
                            err.to_string().as_str(),
                            None,
                        ))
                    }
                }
            }
        }
    }

    async fn update(&self, mail_template_req: MailTemplateModel) -> Result<bool, DomainError> {
        let db = self._db_connection.as_ref();
        let existing = MailTemplate::find_by_id(mail_template_req.id).one(db).await;
        let mail_template_option = match existing {
            Ok(f) => f,
            Err(_) => None,
        };

        let mut mail_template: mail_template::ActiveModel = match mail_template_option {
            Some(f) => f.into(),
            None => {
                return Err(DomainError::new(
                    ErrorType::NotFound,
                    "Flashcard file not found",
                    None,
                ))
            }
        };

        mail_template.updated_by_id = Set(mail_template_req.updated_by_id);
        mail_template.subject = Set(mail_template_req.subject);
        mail_template.body = Set(mail_template_req.body);
        mail_template.is_actived = Set(mail_template_req.is_actived);
        mail_template.is_enabled = Set(mail_template_req.is_enabled);
        mail_template.name = Set(mail_template_req.name);
        mail_template.updated_date = Set(Utc::now().fixed_offset());

        match MailTemplate::update(mail_template).exec(db).await {
            Ok(_) => Ok(true),
            Err(err) => Err(DomainError::new(
                ErrorType::DatabaseError,
                err.to_string().as_str(),
                None,
            )),
        }
    }
}

fn map_entity_to_model(mail_template: mail_template::Model) -> MailTemplateModel {
    MailTemplateModel {
        id: mail_template.id,
        name: mail_template.name,
        subject: mail_template.subject,
        body: mail_template.body,
        created_date: mail_template.created_date.with_timezone(&Utc),
        updated_date: mail_template.updated_date.with_timezone(&Utc),
        created_by_id: mail_template.created_by_id,
        updated_by_id: mail_template.updated_by_id,
        is_actived: mail_template.is_actived,
        is_enabled: mail_template.is_enabled,
    }
}
