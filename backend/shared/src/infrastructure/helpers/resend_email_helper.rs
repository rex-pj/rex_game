use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::infrastructure::helpers::email_provider::EmailProviderTrait;

#[derive(Debug, Serialize)]
struct ResendEmailRequest {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
}

#[derive(Debug, Deserialize)]
struct ResendResponse {
    id: Option<String>,
}

pub struct ResendEmailHelper {
    api_key: String,
    from_email: String,
    from_name: String,
    client: Arc<Client>,
}

impl ResendEmailHelper {
    pub fn new(api_key: String, from_email: String, from_name: String) -> Self {
        Self {
            api_key,
            from_email,
            from_name,
            client: Arc::new(Client::new()),
        }
    }

    pub fn from_env() -> Result<Self, String> {
        let api_key =
            std::env::var("RESEND_API_KEY").map_err(|_| "RESEND_API_KEY not set".to_string())?;
        let from_email = std::env::var("EMAIL_FROM_ADDRESS")
            .map_err(|_| "EMAIL_FROM_ADDRESS not set".to_string())?;
        let from_name =
            std::env::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "qHortus".to_string());

        Ok(Self::new(api_key, from_email, from_name))
    }
}

#[async_trait]
impl EmailProviderTrait for ResendEmailHelper {
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let from = format!("{} <{}>", self.from_name, self.from_email);

        let request = ResendEmailRequest {
            from,
            to: vec![to.to_string()],
            subject: subject.to_string(),
            html: html_body.to_string(),
        };

        let response = self
            .client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: ResendResponse = response.json().await?;
            eprintln!("[Resend] Email sent successfully, id: {:?}", result.id);
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(format!("Resend API error: {}", error_text).into())
        }
    }
}
