use super::{
    MailTemplateCreationDto, MailTemplateDeletionDto, MailTemplateDto, MailTemplateUpdationDto,
    MailTemplateUseCaseTrait,
};
use crate::{domain::models::MailTemplateModel, infrastructure::repositories::MailTemplateRepository};
use chrono::Utc;
use rex_game_shared::{
    InfraError,
    domain::models::page_list_model::PageListModel,
};

#[derive(Clone)]
pub struct MailTemplateUseCase {
    _mail_template_repository: MailTemplateRepository,
}

impl MailTemplateUseCase {
    pub fn new(mail_template_repository: MailTemplateRepository) -> Self {
        Self {
            _mail_template_repository: mail_template_repository,
        }
    }
}

impl MailTemplateUseCaseTrait for MailTemplateUseCase {
    async fn create(&self, mail_template_req: MailTemplateCreationDto) -> Result<i32, InfraError> {
        let model = MailTemplateModel {
            id: 0,
            name: mail_template_req.name,
            subject: mail_template_req.subject,
            body: mail_template_req.body,
            created_by_id: mail_template_req.created_by_id,
            created_date: Utc::now(),
            updated_date: Utc::now(),
            updated_by_id: None,
            is_actived: true,
            is_enabled: true,
        };
        self._mail_template_repository.create(model).await
    }

    async fn update(&self, mail_template_req: MailTemplateUpdationDto) -> Result<bool, InfraError> {
        // Get existing mail template
        let existing = self._mail_template_repository.get_by_id(mail_template_req.id).await?;

        let model = MailTemplateModel {
            id: mail_template_req.id,
            name: mail_template_req.name.unwrap_or(existing.name),
            subject: mail_template_req.subject.unwrap_or(existing.subject),
            body: mail_template_req.body.unwrap_or(existing.body),
            created_by_id: existing.created_by_id,
            created_date: existing.created_date,
            updated_date: Utc::now(),
            updated_by_id: mail_template_req.updated_by_id,
            is_actived: existing.is_actived,
            is_enabled: existing.is_enabled,
        };
        self._mail_template_repository.update(model).await
    }

    async fn delete(&self, mail_template_req: MailTemplateDeletionDto) -> Result<bool, InfraError> {
        self._mail_template_repository.delete(mail_template_req.id).await
    }

    async fn get_list(
        &self,
        page: u64,
        per_page: u64,
        search: String,
    ) -> Result<PageListModel<MailTemplateDto>, InfraError> {
        let result = self._mail_template_repository.get_list(page, per_page, search).await?;
        
        let items = result.items.into_iter().map(|m| MailTemplateDto {
            id: m.id,
            name: m.name,
            subject: m.subject,
            body: m.body,
            created_by_id: m.created_by_id,
            created_date: m.created_date.to_rfc3339(),
            updated_date: m.updated_date.to_rfc3339(),
            updated_by_id: m.updated_by_id,
            is_actived: m.is_actived,
            is_enabled: m.is_enabled,
        }).collect();

        Ok(PageListModel {
            items,
            total_count: result.total_count,
        })
    }

    async fn get_by_id(&self, id: i32) -> Result<MailTemplateDto, InfraError> {
        let model = self._mail_template_repository.get_by_id(id).await?;
        Ok(MailTemplateDto {
            id: model.id,
            name: model.name,
            subject: model.subject,
            body: model.body,
            created_by_id: model.created_by_id,
            created_date: model.created_date.to_rfc3339(),
            updated_date: model.updated_date.to_rfc3339(),
            updated_by_id: model.updated_by_id,
            is_actived: model.is_actived,
            is_enabled: model.is_enabled,
        })
    }

    async fn get_by_name(&self, name: String) -> Result<MailTemplateDto, InfraError> {
        let model = self._mail_template_repository.get_by_name(name).await?;
        Ok(MailTemplateDto {
            id: model.id,
            name: model.name,
            subject: model.subject,
            body: model.body,
            created_by_id: model.created_by_id,
            created_date: model.created_date.to_rfc3339(),
            updated_date: model.updated_date.to_rfc3339(),
            updated_by_id: model.updated_by_id,
            is_actived: model.is_actived,
            is_enabled: model.is_enabled,
        })
    }
}
