use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use rex_game_application::flashcard_types::{
    flashcard_type_creation_dto::FlashcardTypeCreationDto, flashcard_type_dto::FlashcardTypeDto,
    flashcard_type_updation_dto::FlashcardTypeUpdationDto,
    flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
};
use serde::Deserialize;

use crate::app_state::AppStateTrait;

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
        State(_state): State<T>,
        Json(payload): Json<Option<FlashcardTypeCreationDto>>,
    ) -> Json<Option<i32>> {
        match payload {
            Some(req) => {
                let inserted_id = _state
                    .flashcard_type_usecase()
                    .create_flashcard_type(req)
                    .await;

                match inserted_id {
                    Some(id) => Json(Some(id)),
                    None => Json(None),
                }
            }
            None => Json(None),
        }
    }

    pub async fn update_flashcard_type<T: AppStateTrait>(
        State(_state): State<T>,
        Path(id): Path<i32>,
        Json(payload): Json<Option<HashMap<String, String>>>,
    ) -> Json<Option<FlashcardTypeDto>> {
        match payload {
            Some(requests) => {
                if requests.is_empty() {
                    return Json(None);
                }

                let mut updating = FlashcardTypeUpdationDto {
                    description: Some("".to_string()),
                    name: "".to_string(),
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
                    Some(u) => Json(Some(u)),
                    None => Json(None),
                }
            }
            None => Json(None),
        }
    }
}

pub struct FlashcardTypeHandler {}
