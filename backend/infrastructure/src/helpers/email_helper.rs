use mail_send::{mail_builder::MessageBuilder, Credentials, SmtpClientBuilder};
use rex_game_domain::{
    errors::domain_error::{DomainError, ErrorType},
    helpers::email_helper_trait::{EmailHelperTrait, EmailMessage},
};
use std::sync::Arc;

use crate::helpers::configuration_helper::ConfigurationHelper;

#[derive(Clone)]
pub struct EmailHelper {}

impl EmailHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmailHelperTrait for EmailHelper {
    async fn send_email(&self, message: EmailMessage) -> Result<bool, DomainError> {
        let configuration_helper = Arc::new(ConfigurationHelper::new());

        let mut builed_message = MessageBuilder::new().subject(message.subject);

        match &message.to_name {
            Some(to_name) => {
                builed_message = builed_message.to((to_name.as_str(), message.to_email.as_str()));
            }
            None => {
                builed_message = builed_message.to(message.to_email.as_str());
            }
        };

        // Always set the "from" field to avoid MissingFromMail error
        match (&message.from_name, &message.from_email) {
            (Some(from_name), from_email) if !from_email.is_empty() => {
                builed_message = builed_message.from((from_name.as_str(), from_email.as_str()));
            }
            (None, from_email) if !from_email.is_empty() => {
                builed_message = builed_message.from(from_email.as_str());
            }
            _ => {
                return Err(DomainError::new(
                    ErrorType::EmailError,
                    "Missing 'from' email address",
                    None,
                ));
            }
        };

        if let Some(text) = message.text_body {
            builed_message = builed_message.text_body(text)
        }

        if let Some(html) = message.html_body {
            builed_message = builed_message.html_body(html)
        }

        let username = configuration_helper.get_value("email.username");
        let password = configuration_helper.get_value("email.password");
        let host = configuration_helper.get_value("email.host");
        let port = configuration_helper
            .get_value("email.port")
            .parse::<u16>()
            .unwrap_or(587);
        let sent = SmtpClientBuilder::new(host, port)
            .implicit_tls(false)
            .credentials(Credentials::new(username, password))
            .connect()
            .await
            .map_err(|err| {
                DomainError::new(
                    ErrorType::EmailError,
                    "Failed to connect to SMTP server",
                    Some(err.to_string()),
                )
            })?
            // .send_signed(builed_message, &signer)
            .send(builed_message)
            .await;

        match sent {
            Ok(_) => Ok(true),
            Err(e) => Err({
                DomainError::new(
                    ErrorType::EmailError,
                    "Failed to send email",
                    Some(e.to_string()),
                )
            }),
        }
    }
}
