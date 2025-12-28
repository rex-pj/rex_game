use crate::infrastructure::errors::InfraError;
use std::fmt;

/// Application layer errors for business logic and use case orchestration
#[derive(Debug)]
pub enum ApplicationError {
    // ========================================================================
    // Business Validation Errors
    // ========================================================================
    /// Business validation failed
    ValidationFailed(String),

    /// Business rule violation
    BusinessRuleViolated(String),

    /// Invalid input parameters
    InvalidInput(String),

    // ========================================================================
    // Domain Errors
    // ========================================================================
    /// Entity not found
    EntityNotFound { entity: String, id: String },

    /// Duplicate entry (unique constraint violation)
    DuplicateEntry(String),

    /// Entity already exists
    AlreadyExists(String),

    // ========================================================================
    // Authorization & Authentication Errors
    // ========================================================================
    /// User is not authenticated
    Unauthorized(String),

    /// User lacks required permissions
    Forbidden(String),

    /// Invalid credentials
    InvalidCredentials,

    /// Token expired or invalid
    InvalidToken(String),

    // ========================================================================
    // State Errors
    // ========================================================================
    /// Operation not allowed in current state
    InvalidState(String),

    /// Resource conflict
    Conflict(String),

    // ========================================================================
    // Infrastructure Errors (bubbled up)
    // ========================================================================
    /// Infrastructure layer error
    Infrastructure(InfraError),
}

impl ApplicationError {
    // Convenience constructors
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationFailed(message.into())
    }

    pub fn business_rule(message: impl Into<String>) -> Self {
        Self::BusinessRuleViolated(message.into())
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput(message.into())
    }

    pub fn not_found(entity: impl Into<String>, id: impl Into<String>) -> Self {
        Self::EntityNotFound {
            entity: entity.into(),
            id: id.into(),
        }
    }

    pub fn duplicate(message: impl Into<String>) -> Self {
        Self::DuplicateEntry(message.into())
    }

    pub fn already_exists(message: impl Into<String>) -> Self {
        Self::AlreadyExists(message.into())
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::Unauthorized(message.into())
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    pub fn invalid_token(message: impl Into<String>) -> Self {
        Self::InvalidToken(message.into())
    }

    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::InvalidState(message.into())
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            Self::BusinessRuleViolated(msg) => write!(f, "Business rule violated: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::EntityNotFound { entity, id } => {
                write!(f, "{} with id '{}' not found", entity, id)
            }
            Self::DuplicateEntry(msg) => write!(f, "Duplicate entry: {}", msg),
            Self::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            Self::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            Self::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
            Self::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            Self::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::Infrastructure(err) => write!(f, "Infrastructure error: {}", err),
        }
    }
}

impl std::error::Error for ApplicationError {}

// Auto-convert from InfraError to ApplicationError
impl From<InfraError> for ApplicationError {
    fn from(err: InfraError) -> Self {
        match err {
            InfraError::NotFound { entity, id } => Self::EntityNotFound { entity, id },
            other => Self::Infrastructure(other),
        }
    }
}
