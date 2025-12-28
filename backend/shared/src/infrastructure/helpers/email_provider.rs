use crate::{
    domain::helpers::email_helper_trait::{EmailHelperTrait, EmailMessage},
    infrastructure::helpers::{
        configuration_helper::ConfigurationHelper, email_helper::EmailHelper,
        resend_email_helper::ResendEmailHelper,
    },
    InfraError,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for email providers (used by Resend, etc.)
#[async_trait]
pub trait EmailProviderTrait: Send + Sync {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Email provider types supported by the application
#[derive(Debug, Clone, PartialEq)]
pub enum EmailProviderType {
    Smtp,
    Resend,
}

impl From<&str> for EmailProviderType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "resend" => EmailProviderType::Resend,
            _ => EmailProviderType::Smtp,
        }
    }
}

/// Factory for creating email providers based on configuration
pub struct EmailProviderFactory;

impl EmailProviderFactory {
    /// Get the configured email provider type from configuration
    pub fn get_provider_type() -> EmailProviderType {
        let config = ConfigurationHelper::new();
        let provider = config.get_optional("EMAIL_PROVIDER");
        EmailProviderType::from(provider.as_str())
    }

    /// Send email using the configured provider
    pub async fn send_email(message: EmailMessage) -> Result<bool, InfraError> {
        let provider_type = Self::get_provider_type();

        match provider_type {
            EmailProviderType::Resend => {
                let helper = ResendEmailHelper::from_env()
                    .map_err(|e| InfraError::email(format!("Failed to initialize Resend: {}", e)))?;
                let html_body = message
                    .html_body
                    .unwrap_or_else(|| message.text_body.clone().unwrap_or_default());
                helper
                    .send_email(&message.to_email, &message.subject, &html_body)
                    .await
                    .map_err(|e| InfraError::email(format!("Failed to send email via Resend: {}", e)))?;
                Ok(true)
            }
            EmailProviderType::Smtp => {
                let helper = EmailHelper::new();
                helper.send_email(message).await
            }
        }
    }
}

/// Unified email service that uses the factory pattern
#[derive(Clone)]
pub struct EmailService {
    config: Arc<ConfigurationHelper>,
}

impl EmailService {
    pub fn new() -> Self {
        Self {
            config: Arc::new(ConfigurationHelper::new()),
        }
    }

    /// Get default "from" email address from configuration
    pub fn get_default_from_email(&self) -> String {
        self.config.get("EMAIL_FROM_ADDRESS")
    }

    /// Get default "from" name from configuration
    pub fn get_default_from_name(&self) -> Option<String> {
        let name = self.config.get_optional("EMAIL_FROM_NAME");
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    /// Send email with default from address
    pub async fn send(&self, message: EmailMessage) -> Result<bool, InfraError> {
        EmailProviderFactory::send_email(message).await
    }

    /// Create an email message with default from address
    pub fn create_message(
        &self,
        to_email: String,
        to_name: Option<String>,
        subject: String,
        html_body: Option<String>,
        text_body: Option<String>,
    ) -> EmailMessage {
        EmailMessage {
            to_email,
            to_name,
            from_email: self.get_default_from_email(),
            from_name: self.get_default_from_name(),
            subject,
            html_body,
            text_body,
        }
    }
}

impl Default for EmailService {
    fn default() -> Self {
        Self::new()
    }
}
