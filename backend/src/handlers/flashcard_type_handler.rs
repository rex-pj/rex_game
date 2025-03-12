use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use hyper::StatusCode;
use rex_game_application::flashcard_types::{
    flashcard_type_creation_dto::FlashcardTypeCreationDto, flashcard_type_dto::FlashcardTypeDto,
    flashcard_type_updation_dto::FlashcardTypeUpdationDto,
    flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
};
use serde::Deserialize;

use crate::{
    app_state::AppStateTrait, middlewares::auth_middleware::CurrentUser,
    view_models::flashcard_types::flashcard_type_create_request::FlashcardTypeCreateRequest,
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
    ) -> Json<Option<Vec<FlashcardTypeDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let flashcard_types = _state
            .flashcard_type_usecase()
            .get_flashcard_types(params.name, page, page_size)
            .await;
        return match flashcard_types {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }

    pub async fn get_flashcard_type_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> Json<Option<FlashcardTypeDto>> {
        let flashcard = _state
            .flashcard_type_usecase()
            .get_flashcard_type_by_id(id)
            .await;
        return match flashcard {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }

    pub async fn create_flashcard_type<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Json(payload): Json<Option<FlashcardTypeCreateRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let creation_request = FlashcardTypeCreationDto {
            name: req.name,
            description: req.description,
            created_by_id: Some(current_user.id),
            updated_by_id: Some(current_user.id),
        };
        let inserted_id = _state
            .flashcard_type_usecase()
            .create_flashcard_type(creation_request)
            .await;

        match inserted_id {
            Some(id) => Ok(Json(id)),
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn update_flashcard_type<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, String>>>,
    ) -> Result<Json<FlashcardTypeDto>, StatusCode> {
        let requests = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };
        if requests.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let mut updating = FlashcardTypeUpdationDto {
            updated_by_id: Some(current_user.id),
            ..Default::default()
        };

        for (key, value) in &requests {
            if key.to_lowercase() == "name" {
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
            None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct FlashcardTypeHandler {}
