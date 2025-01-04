use axum::{extract::State, Json};
use rex_game_application::flashcards::{
    flashcard_dto::FlashcardDto, flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use serde_json::Value;

use crate::app_state::AppStateTrait;

impl FlashcardHandler {
    pub async fn get_flashcards<T: AppStateTrait>(
        State(_state): State<T>,
        payload: Option<Json<Value>>,
    ) -> Json<Option<Vec<FlashcardDto>>> {
        let flashcard = _state.flashcard_usecase().get_flashcards().await;
        return match flashcard {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }
}

pub struct FlashcardHandler {}
