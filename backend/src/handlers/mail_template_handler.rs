use crate::{
    app_state::AppState,
    validators::validation_helper::ValidationHelper,
    view_models::{
        mail_templates::mail_template_create_request::MailTemplateCreateRequest,
        users::current_user::CurrentUser, HandlerError, HandlerResult,
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_identity::application::usecases::roles::*;
use rex_game_shared::domain::models::PageListModel;
use rex_game_shared::infrastructure::helpers::html_helper_trait::HtmlHelperTrait;

use rex_game_mail_templates::application::{
    MailTemplateCreationDto, MailTemplateDeletionDto, MailTemplateDto, MailTemplateUpdationDto,
    MailTemplateUseCaseTrait,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use validator::{Validate, ValidationErrors};

#[derive(Deserialize)]
pub struct MailTemplateQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
}

impl MailTemplateHandler {
    pub async fn get_mail_templates(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Query(params): Query<MailTemplateQuery>,
    ) -> Result<Json<PageListModel<MailTemplateDto>>, StatusCode> {
        if !current_user
            .roles
            .iter()
            .any(|mail_template| mail_template == ROLE_ROOT_ADMIN)
        {
            return Err(StatusCode::FORBIDDEN);
        }
        let page = params.page.unwrap_or(1);
        let per_page = params.page_size.unwrap_or(10);
        let search = params.name.unwrap_or_default();
        let mail_templates = _state
            .usecases
            .mail_template
            .get_list(page, per_page, search)
            .await;

        return match mail_templates {
            Ok(mut data) => {
                data.items.iter_mut().for_each(|mt| {
                    let plain_text_body = _state.helpers.html.get_plain_text(mt.body.clone());
                    if let Ok(plain_text) = plain_text_body {
                        if plain_text.len() <= 100 {
                            mt.body = plain_text;
                            return;
                        }
                        mt.body = plain_text[..100].to_string() + "...";
                    }
                });

                Ok(Json(data))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        };
    }

    pub async fn get_mail_template_by_id(
        Path(id): Path<i32>,
        State(_state): State<AppState>,
    ) -> HandlerResult<Json<MailTemplateDto>> {
        let mail_template = _state
            .usecases
            .mail_template
            .get_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Mail template not found: {}", err),
                ..Default::default()
            })?;
        Ok(Json(mail_template))
    }

    pub async fn create_mail_template(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Json(payload): Json<Option<MailTemplateCreateRequest>>,
    ) -> HandlerResult<Json<i32>> {
        let req = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        req.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        if !current_user
            .roles
            .iter()
            .any(|mail_template| mail_template == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to create mail templates".to_string(),
                ..Default::default()
            });
        }

        let existing_mail_template = _state
            .usecases
            .mail_template
            .get_by_name(req.name.clone())
            .await;

        if existing_mail_template.is_ok() {
            return Err(HandlerError {
                status: StatusCode::CONFLICT,
                message: "Mail template with the same name already exists".to_string(),
                ..Default::default()
            });
        }

        let new_mail_template = MailTemplateCreationDto {
            name: req.name,
            subject: req.subject,
            body: req.body,
            created_by_id: Some(current_user.id),
        };
        let created_result = _state
            .usecases
            .mail_template
            .create(new_mail_template)
            .await;
        match created_result {
            Ok(created_id) => Ok(Json(created_id)),
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to create mail template".to_string(),
                ..Default::default()
            }),
        }
    }

    pub async fn update_mail_template(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, String>>>,
    ) -> HandlerResult<Json<bool>> {
        let requests = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };
        if requests.is_empty() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Request payload cannot be empty".to_string(),
                ..Default::default()
            });
        }

        if requests.get("name").is_none()
            && requests.get("subject").is_none()
            && requests.get("body").is_none()
            && requests.get("is_enabled").is_none()
        {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: String::from(
                    "At least one of 'name', 'subject', 'body', or 'is_enabled' must be provided",
                ),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|mail_template| mail_template == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to update mail templates".to_string(),
                ..Default::default()
            });
        }

        let mut updating = MailTemplateUpdationDto {
            id,
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
                let name = value.to_string();
                if name.len() < 1 || name.len() > 255 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Title must be between 1 and 255 characters".to_string(),
                        ..Default::default()
                    });
                }
                updating.name = Some(name);
            } else if key.to_lowercase() == "subject" {
                let subject = value.to_string();
                if subject.len() < 10 || subject.len() > 255 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Subject must be between 10 and 255 characters".to_string(),
                        ..Default::default()
                    });
                }
                updating.subject = Some(value.to_string())
            } else if key.to_lowercase() == "body" {
                let body = value.to_string();
                if body.len() < 10 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Body must be at least 10 characters long".to_string(),
                        ..Default::default()
                    });
                }
                updating.body = Some(body)
            }
        }

        let result = _state.usecases.mail_template.update(updating).await;
        return match result {
            Err(_) => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update mail template".to_string(),
                ..Default::default()
            }),
            Ok(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_mail_template(
        State(_state): State<AppState>,
        Extension(current_user): Extension<Arc<CurrentUser>>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        let deletion_req = MailTemplateDeletionDto { id };

        let existing = _state
            .usecases
            .mail_template
            .get_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Mail template not found: {}", err),
                ..Default::default()
            })?;

        if existing.name == ROLE_ROOT_ADMIN {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "Cannot delete the root admin mail template".to_string(),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|mail_template| mail_template == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to delete mail templates".to_string(),
                ..Default::default()
            });
        }

        let is_succeed = _state.usecases.mail_template.delete(deletion_req).await;

        match is_succeed {
            Ok(u) => Ok(Json(u)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to delete mail template".to_string(),
                    ..Default::default()
                })
            }
        }
    }
}

pub struct MailTemplateHandler {}
