use std::future::Future;

use crate::errors::domain_error::DomainError;

pub struct EmailMessage {
    pub to_name: Option<String>,
    pub to_email: String,
    pub from_name: Option<String>,
    pub from_email: String,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
}

pub trait EmailHelperTrait {
    fn send_email(
        &self,
        message: EmailMessage,
    ) -> impl Future<Output = Result<bool, DomainError>> + Send;
}
