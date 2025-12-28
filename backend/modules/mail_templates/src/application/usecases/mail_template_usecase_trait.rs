use super::{MailTemplateCreationDto, MailTemplateDeletionDto, MailTemplateDto, MailTemplateUpdationDto};
use rex_game_shared_kernel::domain::{errors::domain_error::DomainError, models::page_list_model::PageListModel};
use std::future::Future;

pub trait MailTemplateUseCaseTrait {
    fn create(
        &self,
        mail_template_req: MailTemplateCreationDto,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn update(
        &self,
        mail_template_req: MailTemplateUpdationDto,
    ) -> impl Future<Output = Result<bool, DomainError>>;

    fn delete(
        &self,
        mail_template_req: MailTemplateDeletionDto,
    ) -> impl Future<Output = Result<bool, DomainError>>;

    fn get_list(
        &self,
        page: u64,
        per_page: u64,
        search: String,
    ) -> impl Future<Output = Result<PageListModel<MailTemplateDto>, DomainError>>;

    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<MailTemplateDto, DomainError>>;

    fn get_by_name(&self, name: String) -> impl Future<Output = Result<MailTemplateDto, DomainError>>;
}
