use std::collections::HashMap;

use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use config::Value;
use rex_game_application::flashcards::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use serde::Deserialize;

use crate::app_state::AppStateTrait;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u64>,
    page_size: Option<u64>,
}

impl FlashcardHandler {
    pub async fn get_flashcards<T: AppStateTrait>(
        State(_state): State<T>,
        Query(params): Query<Pagination>,
    ) -> Json<Option<Vec<FlashcardDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let flashcard = _state
            .flashcard_usecase()
            .get_flashcards(page, page_size)
            .await;
        return match flashcard {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }

    pub async fn get_flashcard_by_id<T: AppStateTrait>(
        Path(id): Path<i32>,
        State(_state): State<T>,
    ) -> Json<Option<FlashcardDto>> {
        let flashcard = _state.flashcard_usecase().get_flashcard_by_id(id).await;
        return match flashcard {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }

    pub async fn create_flashcard<T: AppStateTrait>(
        State(_state): State<T>,
        mut multipart: Multipart,
    ) -> Json<Option<i32>> {
        let mut flashcard = FlashcardCreationDto {
            name: "".to_string(),
            description: None,
            sub_description: "".to_string(),
            image_data: vec![],
        };
        while let Some(field) = multipart.next_field().await.unwrap() {
            match field.name() {
                Some("name") => {
                    flashcard.name = field.text().await.unwrap();
                }
                Some("description") => {
                    flashcard.description = Some(field.text().await.unwrap());
                }
                Some("sub_description") => {
                    flashcard.sub_description = field.text().await.unwrap();
                }
                Some("image_data") => {
                    flashcard.image_data = field.bytes().await.unwrap().to_vec();
                }
                _ => {}
            }
        }

        let result = _state.flashcard_usecase().create_flashcard(flashcard).await;
        return match result {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }
}

pub struct FlashcardHandler {}
