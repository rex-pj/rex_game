use std::collections::HashMap;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::Serialize;

pub mod authentications;
pub mod flashcard_types;
pub mod flashcards;
pub mod mail_templates;
pub mod permissions;
pub mod roles;
pub mod users;

#[derive(Serialize)]
pub struct HandlerErrorResponse {
    error: String,
    field_errors: Option<HashMap<String, String>>,
}

#[derive(Default)]
pub struct HandlerError {
    pub status: StatusCode,
    pub message: String,
    pub field_errors: Option<HashMap<String, String>>,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let body = Json(HandlerErrorResponse {
            error: self.message,
            field_errors: self.field_errors,
        });
        (self.status, body).into_response()
    }
}

pub type HandlerResult<T> = Result<T, HandlerError>;
