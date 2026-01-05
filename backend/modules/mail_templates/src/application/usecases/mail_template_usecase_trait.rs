use super::{MailTemplateCreationDto, MailTemplateDeletionDto, MailTemplateDto, MailTemplateUpdationDto};
use rex_game_shared::{InfraError, domain::models::page_list_model::PageListModel};
use std::future::Future;

pub trait MailTemplateUseCaseTrait {
    fn create(
        &self,
        mail_template_req: MailTemplateCreationDto,
    ) -> impl Future<Output = Result<i32, InfraError>>;

    fn update(
        &self,
        mail_template_req: MailTemplateUpdationDto,
    ) -> impl Future<Output = Result<bool, InfraError>>;

    fn delete(
        &self,
        mail_template_req: MailTemplateDeletionDto,
    ) -> impl Future<Output = Result<bool, InfraError>>;

    fn get_list(
        &self,
        page: u64,
        per_page: u64,
        search: String,
    ) -> impl Future<Output = Result<PageListModel<MailTemplateDto>, InfraError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<MailTemplateDto, InfraError>>;

    fn get_by_name(&self, name: String) -> impl Future<Output = Result<MailTemplateDto, InfraError>>;
}
