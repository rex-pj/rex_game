use std::future::Future;

use crate::{
    errors::domain_error::DomainError,
    models::{mail_template_model::MailTemplateModel, page_list_model::PageListModel},
};

pub trait MailTemplateRepositoryTrait {
    fn create(
        &self,
        mail_template: MailTemplateModel,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_name(&self, name: &str) -> impl Future<Output = Option<MailTemplateModel>>;
    fn get_by_id(&self, id: i32) -> impl Future<Output = Result<MailTemplateModel, DomainError>>;
    fn get_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> impl Future<Output = Result<Vec<MailTemplateModel>, DomainError>>;
    fn get_paged_list(
        &self,
        name: Option<String>,
        description: Option<String>,
        page_option: u64,
        page_size: Option<u64>,
    ) -> impl Future<Output = Result<PageListModel<MailTemplateModel>, DomainError>>;
    fn update(
        &self,
        mail_template_req: MailTemplateModel,
    ) -> impl Future<Output = Result<bool, DomainError>>;
}
