use std::fmt;

/// Infrastructure layer errors for database, external services, etc.
#[derive(Debug, Clone)]
pub enum InfraError {
    /// Database operation errors (queries, connections, etc.)
    DatabaseError(String),

    /// Email service errors
    EmailError(String),

    /// File storage errors (upload, download, delete)
    FileStorageError(String),

    /// External service errors (APIs, third-party services)
    ExternalServiceError(String),

    /// Entity not found in database
    NotFound { entity: String, id: String },

    /// Connection or network errors
    ConnectionError(String),
}

impl InfraError {
    pub fn database(message: impl Into<String>) -> Self {
        Self::DatabaseError(message.into())
    }

    pub fn email(message: impl Into<String>) -> Self {
        Self::EmailError(message.into())
    }

    pub fn file_storage(message: impl Into<String>) -> Self {
        Self::FileStorageError(message.into())
    }

    pub fn external_service(message: impl Into<String>) -> Self {
        Self::ExternalServiceError(message.into())
    }

    pub fn not_found(entity: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFound {
            entity: entity.into(),
            id: id.into(),
        }
    }

    pub fn connection(message: impl Into<String>) -> Self {
        Self::ConnectionError(message.into())
    }
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::EmailError(msg) => write!(f, "Email error: {}", msg),
            Self::FileStorageError(msg) => write!(f, "File storage error: {}", msg),
            Self::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            Self::NotFound { entity, id } => write!(f, "{} not found: {}", entity, id),
            Self::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl std::error::Error for InfraError {}

// Auto-convert from SeaORM errors
impl From<sea_orm::DbErr> for InfraError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::DatabaseError(err.to_string())
    }
}
