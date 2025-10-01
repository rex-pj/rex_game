use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_application::{
    flashcard_types::{
        flashcard_type_creation_dto::FlashcardTypeCreationDto,
        flashcard_type_dto::FlashcardTypeDto,
        flashcard_type_updation_dto::FlashcardTypeUpdationDto,
        flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
    },
    page_list_dto::PageListDto,
    roles::roles::ROLE_ROOT_ADMIN,
};
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    app_state::AppStateTrait,
    validators::validation_helper::ValidationHelper,
    view_models::{
        flashcard_types::flashcard_type_create_request::FlashcardTypeCreateRequest,
        users::current_user::CurrentUser, HandlerError, HandlerResult,
    },
};

#[derive(Deserialize)]
pub struct FlashcardQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
}

impl FlashcardTypeHandler {
    pub async fn get_flashcard_types<T: AppStateTrait>(
        State(_state): State<T>,
        Query(params): Query<FlashcardQuery>,
    ) -> HandlerResult<Json<PageListDto<FlashcardTypeDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let flashcard_types = _state
            .flashcard_type_usecase()
            .get_flashcard_types(params.name, page, page_size)
            .await;
        return match flashcard_types {
            Ok(data) => Ok(Json(data)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to fetch flashcard types".to_string(),
                    ..Default::default()
                })
            }
        };
    }

    pub async fn get_flashcard_type_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<FlashcardTypeDto>> {
        let flashcard = _state
            .flashcard_type_usecase()
            .get_flashcard_type_by_id(id)
            .await;
        return match flashcard {
            None => Err(HandlerError {
                status: StatusCode::NOT_FOUND,
                message: "Flashcard type not found".to_string(),
                ..Default::default()
            }),
            Some(i) => Ok(Json(i)),
        };
    }

    pub async fn create_flashcard_type<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Json(payload): Json<Option<FlashcardTypeCreateRequest>>,
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

        if req.name.is_empty() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Name cannot be empty".to_string(),
                ..Default::default()
            });
        }

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to create flashcard types".to_string(),
                ..Default::default()
            });
        }

        match req.description {
            Some(description) if description.is_empty() => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Description cannot be empty".to_string(),
                    ..Default::default()
                });
            }
            _ => {}
        }

        let creation_request = FlashcardTypeCreationDto {
            name: req.name,
            description: req.description,
            created_by_id: current_user.id,
            updated_by_id: current_user.id,
        };
        let inserted_id = _state
            .flashcard_type_usecase()
            .create_flashcard_type(creation_request)
            .await;

        match inserted_id {
            Some(id) => Ok(Json(id)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to create flashcard type".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn update_flashcard_type<T: AppStateTrait>(
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

        if requests.get("name").is_none() && requests.get("description").is_none() {
            return Err(HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "At least one of 'name' or 'description' must be provided".to_string(),
                ..Default::default()
            });
        }

        let mut updating = FlashcardTypeUpdationDto {
            updated_by_id: current_user.id,
            ..Default::default()
        };

        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to update flashcard types".to_string(),
                ..Default::default()
            });
        }

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
                if value.len() < 1 || value.len() > 255 {
                    return Err(HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Title must be between 1 and 255 characters".to_string(),
                        ..Default::default()
                    });
                }
                updating.name = value.to_string();
            } else if key.to_lowercase() == "description" {
                updating.description = Some(value.to_string())
            }
        }

        let updated = _state
            .flashcard_type_usecase()
            .update_flashcard_type(id, updating)
            .await;

        match updated {
            Some(u) => Ok(Json(u)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: "Failed to update flashcard type".to_string(),
                    ..Default::default()
                })
            }
        }
    }

    pub async fn delete_flashcard_type<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
    ) -> HandlerResult<Json<u64>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to delete flashcard types".to_string(),
                ..Default::default()
            });
        }
        let deleted_numbers = _state
            .flashcard_type_usecase()
            .delete_flashcard_type_by_id(id)
            .await;

        match deleted_numbers {
            Some(u) => Ok(Json(u)),
            None => {
                return Err(HandlerError {
                    status: StatusCode::NOT_FOUND,
                    message: "Flashcard type not found".to_string(),
                    ..Default::default()
                })
            }
        }
    }
}

pub struct FlashcardTypeHandler {}
