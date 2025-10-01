use super::{
    mail_template_dto::MailTemplateDto, mail_template_usecase_trait::MailTemplateUseCaseTrait,
};
use crate::{
    errors::application_error::{ApplicationError, ApplicationErrorKind},
    mail_templates::{
        mail_template_creation_dto::MailTemplateCreationDto,
        mail_template_deletion_dto::MailTemplateDeletionDto,
        mail_template_updation_dto::MailTemplateUpdationDto,
    },
    page_list_dto::PageListDto,
};
use chrono::Utc;
use rex_game_domain::{
    models::mail_template_model::MailTemplateModel,
    repositories::mail_template_repository_trait::MailTemplateRepositoryTrait,
};

#[derive(Clone)]
pub struct MailTemplateUseCase<R>
where
    R: MailTemplateRepositoryTrait,
{
    _mail_template_repository: R,
}

impl<R> MailTemplateUseCase<R>
where
    R: MailTemplateRepositoryTrait,
{
    pub fn new(mail_template_repository: R) -> Self {
        Self {
            _mail_template_repository: mail_template_repository,
        }
    }
}

impl<R> MailTemplateUseCaseTrait for MailTemplateUseCase<R>
where
    R: MailTemplateRepositoryTrait,
{
    async fn get_mail_templates(
        &self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> Result<PageListDto<MailTemplateDto>, ApplicationError> {
        let mail_templates_result = self
            ._mail_template_repository
            .get_paged_list(name, description, page, page_size_option)
            .await;
        match mail_templates_result {
            Ok(i) => {
                let items = i
                    .items
                    .into_iter()
                    .map(|f| MailTemplateDto {
                        id: f.id,
                        name: f.name,
                        subject: f.subject,
                        body: f.body,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                        is_enabled: f.is_enabled,
                    })
                    .collect();
                let page_size: u64 = page_size_option.unwrap_or(i.total_count);
                Ok(PageListDto {
                    items,
                    total_count: i.total_count,
                    page,
                    page_size,
                })
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get mail_templates",
                None,
            )),
        }
    }

    async fn get_mail_templates_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> Result<Vec<MailTemplateDto>, ApplicationError> {
        let mail_templates_result = self._mail_template_repository.get_by_ids(ids).await;
        match mail_templates_result {
            Ok(i) => {
                let items = i
                    .into_iter()
                    .map(|f| MailTemplateDto {
                        id: f.id,
                        name: f.name,
                        subject: f.subject,
                        body: f.body,
                        created_date: f.created_date.with_timezone(&Utc),
                        updated_date: f.updated_date.with_timezone(&Utc),
                        created_by_id: f.created_by_id,
                        updated_by_id: f.updated_by_id,
                        is_enabled: f.is_enabled,
                    })
                    .collect();
                Ok(items)
            }
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Failed to get mail_templates by ids",
                None,
            )),
        }
    }

    async fn get_mail_template_by_name(&self, name: &str) -> Option<MailTemplateDto> {
        let existing = self._mail_template_repository.get_by_name(name).await;
        match existing {
            Some(f) => Some(MailTemplateDto {
                id: f.id,
                name: f.name,
                subject: f.subject,
                body: f.body,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                created_by_id: f.created_by_id,
                updated_by_id: f.updated_by_id,
                is_enabled: f.is_enabled,
            }),
            None => None,
        }
    }

    async fn get_mail_template_by_id(&self, id: i32) -> Result<MailTemplateDto, ApplicationError> {
        let existing = self._mail_template_repository.get_by_id(id).await;
        match existing {
            Ok(f) => Ok(MailTemplateDto {
                id: f.id,
                name: f.name,
                subject: f.subject,
                body: f.body,
                created_by_id: f.created_by_id,
                created_date: f.created_date.with_timezone(&Utc),
                updated_date: f.updated_date.with_timezone(&Utc),
                updated_by_id: f.updated_by_id,
                is_enabled: f.is_enabled,
            }),
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
        }
    }

    async fn create_mail_template(
        &self,
        mail_template_req: MailTemplateCreationDto,
    ) -> Result<i32, ApplicationError> {
        let active_mail_template = MailTemplateModel {
            name: mail_template_req.name,
            subject: mail_template_req.subject,
            created_by_id: mail_template_req.created_by_id,
            updated_by_id: mail_template_req.updated_by_id,
            ..Default::default()
        };

        let created = self
            ._mail_template_repository
            .create(active_mail_template)
            .await;

        match created {
            Err(_) => Err(ApplicationError::new(
                ApplicationErrorKind::DatabaseError,
                "Database error",
                None,
            )),
            Ok(i) => Ok(i),
        }
    }

    async fn update_mail_template<'a>(
        &'a self,
        id: i32,
        mail_template_req: MailTemplateUpdationDto,
    ) -> Option<bool> {
        let existing = self._mail_template_repository.get_by_id(id).await;
        match existing {
            Ok(mut exist) => {
                if let Some(name) = mail_template_req.name {
                    exist.name = name
                };
                if let Some(subject) = mail_template_req.subject {
                    exist.subject = subject
                };
                if let Some(body) = mail_template_req.body {
                    exist.body = body
                };
                if let Some(is_actived) = mail_template_req.is_actived {
                    exist.is_actived = is_actived
                };
                if let Some(is_enabled) = mail_template_req.is_enabled {
                    exist.is_enabled = is_enabled
                };
                let updated = self._mail_template_repository.update(exist).await;
                match updated {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    async fn delete_mail_template_by_id(
        &self,
        id: i32,
        delete_req: MailTemplateDeletionDto,
    ) -> Option<bool> {
        let updation = MailTemplateUpdationDto {
            updated_by_id: delete_req.updated_by_id,
            is_actived: Some(false),
            ..Default::default()
        };
        Some(self.update_mail_template(id, updation).await?)
    }
}
