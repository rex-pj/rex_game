use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::collections::HashMap;

/// Standardized error response structure
#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, error: String, message: String) -> Self {
        Self {
            status: status.as_u16(),
            error,
            message,
            field_errors: None,
            trace_id: None,
        }
    }

    pub fn with_field_errors(mut self, errors: HashMap<String, Vec<String>>) -> Self {
        self.field_errors = Some(errors);
        self
    }

    pub fn with_trace_id(mut self, trace_id: String) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    /// Create a generic internal server error (hiding internal details)
    pub fn internal_server_error() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: "Internal Server Error".to_string(),
            message: "An unexpected error occurred. Please try again later.".to_string(),
            field_errors: None,
            trace_id: None,
        }
    }

    /// Create a validation error response
    pub fn validation_error(field_errors: HashMap<String, Vec<String>>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Validation Error".to_string(),
            message: "One or more validation errors occurred.".to_string(),
            field_errors: Some(field_errors),
            trace_id: None,
        }
    }

    /// Create an unauthorized error
    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED.as_u16(),
            error: "Unauthorized".to_string(),
            message: "Authentication is required to access this resource.".to_string(),
            field_errors: None,
            trace_id: None,
        }
    }

    /// Create a forbidden error
    pub fn forbidden() -> Self {
        Self {
            status: StatusCode::FORBIDDEN.as_u16(),
            error: "Forbidden".to_string(),
            message: "You do not have permission to access this resource.".to_string(),
            field_errors: None,
            trace_id: None,
        }
    }

    /// Create a not found error
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND.as_u16(),
            error: "Not Found".to_string(),
            message: "The requested resource was not found.".to_string(),
            field_errors: None,
            trace_id: None,
        }
    }

    /// Create a conflict error
    pub fn conflict(message: String) -> Self {
        Self {
            status: StatusCode::CONFLICT.as_u16(),
            error: "Conflict".to_string(),
            message,
            field_errors: None,
            trace_id: None,
        }
    }

    /// Create a bad request error
    pub fn bad_request(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST.as_u16(),
            error: "Bad Request".to_string(),
            message,
            field_errors: None,
            trace_id: None,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}

/// Transform database errors into user-friendly messages
pub fn transform_db_error(error: &str) -> ErrorResponse {
    // Log the full error internally (use tracing in production)
    eprintln!("[DB Error] {}", error);

    // Check for common database errors and return appropriate responses
    if error.contains("duplicate key") || error.contains("unique constraint") {
        return ErrorResponse::conflict("A record with this information already exists.".to_string());
    }

    if error.contains("foreign key constraint") {
        return ErrorResponse::bad_request("Cannot complete this operation due to related records.".to_string());
    }

    if error.contains("not found") || error.contains("no rows") {
        return ErrorResponse::not_found();
    }

    if error.contains("connection") || error.contains("timeout") {
        return ErrorResponse::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "Service Unavailable".to_string(),
            "Database is temporarily unavailable. Please try again later.".to_string(),
        );
    }

    // For unknown errors, return generic message (don't expose internal details)
    ErrorResponse::internal_server_error()
}

/// Transform application errors into user-friendly messages
pub fn transform_app_error(error_type: &str, message: &str) -> ErrorResponse {
    match error_type {
        "NotFound" => ErrorResponse::not_found(),
        "InvalidInput" => ErrorResponse::bad_request(message.to_string()),
        "InvalidCredentials" => ErrorResponse::new(
            StatusCode::UNAUTHORIZED,
            "Invalid Credentials".to_string(),
            "The email or password you entered is incorrect.".to_string(),
        ),
        "InvalidToken" => ErrorResponse::unauthorized(),
        "Unauthorized" => ErrorResponse::unauthorized(),
        "Forbidden" => ErrorResponse::forbidden(),
        "Conflict" => ErrorResponse::conflict(message.to_string()),
        _ => {
            // Log unknown error types
            eprintln!("[Unknown App Error] type={}, message={}", error_type, message);
            ErrorResponse::internal_server_error()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse::internal_server_error();
        assert_eq!(error.status, 500);
        assert_eq!(error.error, "Internal Server Error");
    }

    #[test]
    fn test_transform_db_duplicate_key() {
        let error = transform_db_error("duplicate key value violates unique constraint");
        assert_eq!(error.status, 409);
    }

    #[test]
    fn test_transform_db_not_found() {
        let error = transform_db_error("no rows returned by SELECT");
        assert_eq!(error.status, 404);
    }
}
