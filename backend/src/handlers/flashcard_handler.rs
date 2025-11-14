use crate::{
    app_state::AppStateTrait,
    helpers::http_helper::HttpHelper,
    validators::{validate_content_type, validate_file_size, validation_helper::ValidationHelper},
    view_models::{
        flashcards::flashcard_request::FlashcardRequest, users::current_user::CurrentUser,
        HandlerError, HandlerResult,
    },
};
use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Response,
    Extension, Json,
};
use rex_game_application::{
    flashcard_types::flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
    flashcards::{
        flashcard_creation_dto::FlashcardCreationDto, flashcard_detail_dto::FlashcardDetailDto,
        flashcard_dto::FlashcardDto, flashcard_updation_dto::FlashcardUpdationDto,
        flashcard_usecase_trait::FlashcardUseCaseTrait,
    },
    page_list_dto::PageListDto,
    roles::roles::ROLE_ROOT_ADMIN,
};
use serde::Deserialize;
use std::sync::Arc;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize)]
pub struct FlashcardQuery {
    page: Option<u64>,
    page_size: Option<u64>,
    type_name: Option<String>,
}

impl FlashcardHandler {
    pub async fn get_flashcards<T: AppStateTrait>(
        State(_state): State<T>,
        Query(params): Query<FlashcardQuery>,
    ) -> HandlerResult<Json<PageListDto<FlashcardDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let flashcards = _state
            .flashcard_usecase()
            .get_paged_list(params.type_name, page, page_size)
            .await
            .map_err(|_| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to fetch flashcards".to_string(),
                ..Default::default()
            })?;
        return Ok(Json(flashcards));
    }

    pub async fn get_flashcard_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> HandlerResult<Json<FlashcardDetailDto>> {
        let flashcard = match _state.flashcard_usecase().get_flashcard_by_id(id).await {
            Some(flashcard) => flashcard,
            None => {
                return Err(HandlerError {
                    status: StatusCode::NOT_FOUND,
                    message: "Flashcard not found".to_string(),
                    ..Default::default()
                })
            }
        };

        let flashcard_types = match _state
            .flashcard_type_usecase()
            .get_flashcard_type_by_flashcard_id(id)
            .await
        {
            None => Vec::new(),
            Some(types) => types,
        };

        Ok(Json(FlashcardDetailDto {
            id: flashcard.id,
            name: flashcard.name,
            description: flashcard.description,
            sub_description: flashcard.sub_description,
            created_date: flashcard.created_date,
            updated_date: flashcard.updated_date,
            image_id: flashcard.image_id,
            flashcard_types: flashcard_types.into_iter().map(|f| f.into()).collect(),
        }))
    }

    pub async fn get_flashcard_image<T: AppStateTrait>(
        Path(file_id): Path<i32>,
        State(_state): State<T>,
    ) -> HandlerResult<Response<Body>> {
        let flashcard_file = _state
            .flashcard_usecase()
            .get_image_by_file_id(file_id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to fetch flashcard image: {}", err.message),
                ..Default::default()
            })?;

        let response =
            HttpHelper::build_file_respone(flashcard_file.data, &flashcard_file.content_type)
                .map_err(|err| HandlerError {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to build response: {}", err.message),
                    ..Default::default()
                })?;

        Ok(response)
    }

    pub async fn create_flashcard<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        mut multipart: Multipart,
    ) -> HandlerResult<Json<i32>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to create flashcards".to_string(),
                ..Default::default()
            });
        }
        let mut flashcard_req = FlashcardRequest {
            ..Default::default()
        };

        while let Some(field) = multipart.next_field().await.unwrap() {
            let field_name = field.name().unwrap_or("").to_string();
            if field_name.contains("type_ids") {
                let type_id =
                    field
                        .text()
                        .await
                        .unwrap()
                        .parse::<i32>()
                        .map_err(|err| HandlerError {
                            status: StatusCode::BAD_REQUEST,
                            message: format!("Invalid type_id: {}", err),
                            ..Default::default()
                        })?;
                flashcard_req.type_ids.push(type_id);
                continue;
            }

            // Process other fields
            match field.name() {
                Some("name") => {
                    flashcard_req.name = field.text().await.unwrap();
                }
                Some("description") => {
                    flashcard_req.description = Some(field.text().await.unwrap());
                }
                Some("sub_description") => {
                    flashcard_req.sub_description = Some(field.text().await.unwrap());
                }
                Some("image_data") => {
                    let content_type = field.content_type().unwrap_or_default().to_string();
                    validate_content_type(&content_type).map_err(|_| HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Title must be between 1 and 255 characters".to_string(),
                        ..Default::default()
                    })?;

                    flashcard_req.content_type = content_type;
                    flashcard_req.file_name = field.file_name().unwrap_or_default().to_string();
                    let bytes_data = field.bytes().await;
                    match bytes_data {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                flashcard_req.image_data = Some(bytes.to_vec());
                            }
                        }
                        Err(_) => {}
                    }
                }
                _ => {}
            }
        }

        flashcard_req.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let new_flashcard = FlashcardCreationDto {
            name: flashcard_req.name,
            description: flashcard_req.description,
            sub_description: flashcard_req.sub_description,
            content_type: flashcard_req.content_type,
            file_name: flashcard_req.file_name,
            image_data: flashcard_req.image_data,
            type_ids: flashcard_req.type_ids,
            created_by_id: current_user.id,
            updated_by_id: current_user.id,
        };

        let id = _state
            .flashcard_usecase()
            .create_flashcard(new_flashcard)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to create flashcard: {}", err.message),

                ..Default::default()
            })?;
        Ok(Json(id))
    }

    pub async fn update_flashcard<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
        mut multipart: Multipart,
    ) -> HandlerResult<Json<bool>> {
        if !current_user
            .roles
            .iter()
            .any(|role| role == ROLE_ROOT_ADMIN)
        {
            return Err(HandlerError {
                status: StatusCode::FORBIDDEN,
                message: "You do not have permission to update flashcards".to_string(),
                ..Default::default()
            });
        }

        let mut flashcard = FlashcardUpdationDto {
            updated_by_id: current_user.id,
            ..Default::default()
        };

        while let Some(field) = multipart.next_field().await.unwrap() {
            let field_name = field.name().unwrap_or("").to_string();
            if field_name.contains("type_ids") {
                let type_id =
                    field
                        .text()
                        .await
                        .unwrap()
                        .parse::<i32>()
                        .map_err(|err| HandlerError {
                            status: StatusCode::BAD_REQUEST,
                            message: format!("Invalid type_id: {}", err),
                            ..Default::default()
                        })?;

                flashcard.type_ids.get_or_insert(Vec::new()).push(type_id);
                continue;
            }

            match field.name() {
                Some("name") => {
                    let name = field.text().await.unwrap();
                    if name.len() < 1 || name.len() > 255 {
                        return Err(HandlerError {
                            status: StatusCode::BAD_REQUEST,
                            message: "Title must be between 1 and 255 characters".to_string(),
                            ..Default::default()
                        });
                    }
                    flashcard.name = Some(name);
                }
                Some("description") => {
                    flashcard.description = Some(field.text().await.unwrap());
                }
                Some("sub_description") => {
                    flashcard.sub_description = Some(field.text().await.unwrap());
                }
                Some("image_data") => {
                    let content_type = field.content_type().unwrap_or_default().to_string();
                    validate_content_type(&content_type).map_err(|_| HandlerError {
                        status: StatusCode::BAD_REQUEST,
                        message: "Title must be between 1 and 255 characters".to_string(),
                        ..Default::default()
                    })?;

                    flashcard.content_type = Some(content_type);
                    let file_name = field.file_name().unwrap_or_default().to_string();
                    if file_name.len() < 1 || file_name.len() > 255 {
                        return Err(HandlerError {
                            status: StatusCode::BAD_REQUEST,
                            message: "File name must be between 1 and 255 characters".to_string(),
                            ..Default::default()
                        });
                    }
                    flashcard.file_name = Some(file_name);
                    let bytes_data = field.bytes().await;
                    match bytes_data {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                let byte_data = bytes.to_vec();
                                validate_file_size(&byte_data).map_err(|_| HandlerError {
                                    status: StatusCode::BAD_REQUEST,
                                    message: "File size must be less than 2MB.".to_string(),
                                    ..Default::default()
                                })?;
                                flashcard.image_data = Some(byte_data);
                            }
                        }
                        Err(_) => {}
                    }
                }
                _ => {}
            }
        }

        _state
            .flashcard_usecase()
            .update_flashcard(id, flashcard)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to update flashcard: {}", err.message),
                ..Default::default()
            })?;
        Ok(Json(true))
    }

    pub async fn delete_flashcard<T: AppStateTrait>(
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
                message: "You do not have permission to delete flashcards".to_string(),
                ..Default::default()
            });
        }

        let deleted_numbers = _state
            .flashcard_usecase()
            .delete_flashcard_by_id(id)
            .await
            .map_err(|err| HandlerError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to delete flashcard: {}", err.message),
                ..Default::default()
            })?;

        Ok(Json(deleted_numbers))
    }
}

pub struct FlashcardHandler {}
