use crate::{
    errors::application_error::ApplicationError,
    mail_templates::{
        mail_template_creation_dto::MailTemplateCreationDto,
        mail_template_deletion_dto::MailTemplateDeletionDto,
        mail_template_updation_dto::MailTemplateUpdationDto,
    },
    page_list_dto::PageListDto,
};

use super::mail_template_dto::MailTemplateDto;
use std::future::Future;

pub trait MailTemplateUseCaseTrait {
    fn get_mail_template_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<MailTemplateDto, ApplicationError>>;
    fn get_mail_template_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Option<MailTemplateDto>>;
    fn get_mail_templates<'a>(
        &'a self,
        name: Option<String>,
        description: Option<String>,
        page: u64,
        page_size_option: Option<u64>,
    ) -> impl Future<Output = Result<PageListDto<MailTemplateDto>, ApplicationError>>;
    fn get_mail_templates_by_ids(
        &self,
        ids: Vec<i32>,
    ) -> impl Future<Output = Result<Vec<MailTemplateDto>, ApplicationError>>;
    fn update_mail_template<'a>(
        &'a self,
        id: i32,
        mail_template_req: MailTemplateUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
    fn create_mail_template(
        &self,
        mail_template_req: MailTemplateCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn delete_mail_template_by_id(
        &self,
        id: i32,
        delete_req: MailTemplateDeletionDto,
    ) -> impl Future<Output = Option<bool>>;
}
