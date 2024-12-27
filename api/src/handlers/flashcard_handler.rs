use axum::{extract::State, Json};
use rex_game_application::flashcards::{
    flashcard_dto::FlashcardDto, t_flashcard_usecase::TFlashcardUseCase,
};
use serde_json::Value;

use crate::app_state::AppState;

impl FlashcardHandler {
    pub async fn get_flashcard<S: AppState>(
        State(app_state): State<S>,
        payload: Option<Json<Value>>,
    ) -> Json<FlashcardDto> {
        if let Some(_payload) = payload {
            let flashcard = app_state.flashcard_usecase().get_flashcard();
            return match flashcard {
                None => {
                    let response = FlashcardDto::new();
                    return Json(response);
                }
                Some(i) => Json(i),
            };
        }

        let response = FlashcardDto::new();
        return Json(response);
    }
}

pub struct FlashcardHandler {}
