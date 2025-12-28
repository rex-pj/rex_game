use std::fmt;
use rex_game_shared_kernel::domain::errors::domain_error::DomainError;

#[derive(Debug, Clone)]
pub enum ApplicationErrorKind {
    ValidationError,
    NotFound,
    Unauthorized,
    Forbidden,
    Conflict,
    InternalError,
    InvalidInput,
}

#[derive(Debug)]
pub struct ApplicationError {
    pub kind: ApplicationErrorKind,
    pub message: String,
    pub details: Option<String>,
}

impl ApplicationError {
    pub fn new(kind: ApplicationErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApplicationError {}

impl From<DomainError> for ApplicationError {
    fn from(err: DomainError) -> Self {
        ApplicationError::new(
            ApplicationErrorKind::InternalError,
            err.message,
        ).with_details(err.details.unwrap_or_default())
    }
}
