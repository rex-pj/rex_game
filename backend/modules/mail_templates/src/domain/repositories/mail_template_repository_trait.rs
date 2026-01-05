use crate::domain::models::MailTemplateModel;
use rex_game_shared::{
    InfraError,
    domain::models::page_list_model::PageListModel,
};
use async_trait::async_trait;

#[async_trait]
pub trait MailTemplateRepositoryTrait: Send + Sync {
    async fn create(&self, mail_template_req: MailTemplateModel) -> Result<i32, InfraError>;
    async fn update(&self, mail_template_id: i32, mail_template_req: MailTemplateModel) -> Result<bool, InfraError>;
    async fn get_by_id(&self, mail_template_id: i32) -> Result<MailTemplateModel, InfraError>;
    async fn get_by_name(&self, name: String) -> Result<MailTemplateModel, InfraError>;
    async fn get_list(
        &self,
        page: u64,
        page_size: u64,
        name: Option<String>,
    ) -> Result<PageListModel<MailTemplateModel>, InfraError>;
    async fn delete(&self, mail_template_id: i32) -> Result<bool, InfraError>;
}
