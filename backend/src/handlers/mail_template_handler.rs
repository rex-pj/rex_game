use crate::{
    app_state::AppStateTrait,
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
use rex_game_application::{
    mail_templates::{
        mail_template_creation_dto::MailTemplateCreationDto,
        mail_template_deletion_dto::MailTemplateDeletionDto, mail_template_dto::MailTemplateDto,
        mail_template_updation_dto::MailTemplateUpdationDto,
        mail_template_usecase_trait::MailTemplateUseCaseTrait,
    },
    page_list_dto::PageListDto,
    roles::roles::ROLE_ROOT_ADMIN,
};
use rex_game_infrastructure::helpers::html_helper_trait::HtmlHelperTrait;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use validator::{Validate, ValidationErrors};

#[derive(Deserialize)]
pub struct MailTemplateQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    subject: Option<String>,
}

impl MailTemplateHandler {
    pub async fn get_mail_templates<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Query(params): Query<MailTemplateQuery>,
    ) -> HandlerResult<Json<PageListDto<MailTemplateDto>>> {
        if !current_user
            .roles
            .iter()
            .any(|mail_template| mail_template == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to view mail templates".to_string(),
                ..Default::default()
            });
        }
        let page = params.page.unwrap_or(1);
        let mail_templates = _state
            .mail_template_usecase()
            .get_mail_templates(params.name, params.subject, page, params.page_size)
            .await;

        return match mail_templates {
            Ok(mut data) => {
                data.items.iter_mut().for_each(|mt| {
                    let plain_text_body = _state.html_helper().get_plain_text(mt.body.clone());
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
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch mail templates".to_string(),
                    ..Default::default()
                })
            }
        };
    }

    pub async fn get_mail_template_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<MailTemplateDto>> {
        let mail_template = _state
            .mail_template_usecase()
            .get_mail_template_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Mail template not found: {}", err.message),
                ..Default::default()
            })?;
        Ok(Json(mail_template))
    }

    pub async fn create_mail_template<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
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
            .mail_template_usecase()
            .get_mail_template_by_name(req.name.as_str())
            .await;

        if let Some(_) = existing_mail_template {
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
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };
        let created_result = _state
            .mail_template_usecase()
            .create_mail_template(new_mail_template)
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

    pub async fn update_mail_template<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
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
            updated_by_id: current_user.id,
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
            } else if key.to_lowercase() == "is_enabled" {
                if let Ok(is_enabled) = value.parse::<bool>() {
                    updating.is_enabled = Some(is_enabled)
                }
            }
        }

        let result = _state
            .mail_template_usecase()
            .update_mail_template(id, updating)
            .await;
        return match result {
            None => Err(HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update mail template".to_string(),
                ..Default::default()
            }),
            Some(_) => Ok(Json(true)),
        };
    }

    pub async fn delete_mail_template<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<bool>> {
        let deletion_req = MailTemplateDeletionDto {
            updated_by_id: current_user.id,
        };

        let existing = _state
            .mail_template_usecase()
            .get_mail_template_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::NOT_FOUND,
                message: format!("Mail template not found: {}", err.message),
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

        let is_succeed = _state
            .mail_template_usecase()
            .delete_mail_template_by_id(id, deletion_req)
            .await;

        match is_succeed {
            Some(u) => Ok(Json(u)),
            None => {
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
