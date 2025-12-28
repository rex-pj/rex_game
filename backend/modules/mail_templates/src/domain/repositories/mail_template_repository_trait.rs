use crate::domain::models::MailTemplateModel;
use rex_game_shared_kernel::domain::{
    errors::domain_error::DomainError,
    models::page_list_model::PageListModel,
};
use async_trait::async_trait;

#[async_trait]
pub trait MailTemplateRepositoryTrait: Send + Sync {
    async fn create(&self, mail_template_req: MailTemplateModel) -> Result<i32, DomainError>;
    async fn update(&self, mail_template_id: i32, mail_template_req: MailTemplateModel) -> Result<bool, DomainError>;
    async fn get_by_id(&self, mail_template_id: i32) -> Result<MailTemplateModel, DomainError>;
    async fn get_by_name(&self, name: String) -> Result<MailTemplateModel, DomainError>;
    async fn get_list(
        &self,
        page: u64,
        page_size: u64,
        name: Option<String>,
    ) -> Result<PageListModel<MailTemplateModel>, DomainError>;
    async fn delete(&self, mail_template_id: i32) -> Result<bool, DomainError>;
}
