use crate::{
    domain::helpers::email_helper_trait::{EmailHelperTrait, EmailMessage},
    InfraError,
};
use mail_send::{mail_builder::MessageBuilder, Credentials, SmtpClientBuilder};
use std::sync::Arc;

use crate::infrastructure::helpers::configuration_helper::ConfigurationHelper;

#[derive(Clone)]
pub struct EmailHelper {}

impl EmailHelper {
    pub fn new() -> Self {
        Self {}
    }
}

impl EmailHelperTrait for EmailHelper {
    async fn send_email(&self, message: EmailMessage) -> Result<bool, InfraError> {
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
                return Err(InfraError::EmailError(
                    "Missing 'from' email address".to_string(),
                ));
            }
        };

        if let Some(text) = message.text_body {
            builed_message = builed_message.text_body(text)
        }

        if let Some(html) = message.html_body {
            builed_message = builed_message.html_body(html)
        }

        let username = configuration_helper.get("SMTP_USERNAME");
        let password = configuration_helper.get("SMTP_PASSWORD");
        let host = configuration_helper.get("SMTP_HOST");
        let port = configuration_helper
            .get_optional("SMTP_PORT")
            .parse::<u16>()
            .unwrap_or(587);
        let sent = SmtpClientBuilder::new(host, port)
            .implicit_tls(false)
            .credentials(Credentials::new(username, password))
            .connect()
            .await
            .map_err(|_| InfraError::EmailError("Failed to connect to SMTP server".to_string()))?
            // .send_signed(builed_message, &signer)
            .send(builed_message)
            .await;

        match sent {
            Ok(_) => Ok(true),
            Err(_) => Err(InfraError::EmailError("Failed to send email".to_string())),
        }
    }
}
