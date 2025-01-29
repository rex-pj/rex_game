use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{Response, Result},
    Json,
};
use rex_game_application::flashcards::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_updation_dto::FlashcardUpdationDto, flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use serde::Deserialize;

use crate::{app_state::AppStateTrait, helpers::http_helper::HttpHelper};

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
    ) -> Json<Option<Vec<FlashcardDto>>> {
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(10);
        let flashcards = _state
            .flashcard_usecase()
            .get_flashcards(params.type_name, page, page_size)
            .await;
        return match flashcards {
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

    pub async fn get_flashcard_image<T: AppStateTrait>(
        Path(file_id): Path<i32>,
        State(_state): State<T>,
    ) -> Result<Response<Body>, StatusCode> {
        let flashcard_data = _state
            .flashcard_usecase()
            .get_image_by_file_id(file_id)
            .await;
        match flashcard_data {
            None => Err(StatusCode::NOT_FOUND),
            Some(file_data) => {
                let response = HttpHelper::build_file_respone(file_data);

                return Ok(response);
            }
        }
    }

    pub async fn create_flashcard<T: AppStateTrait>(
        State(_state): State<T>,
        mut multipart: Multipart,
    ) -> Json<Option<i32>> {
        let mut flashcard = FlashcardCreationDto {
            name: "".to_string(),
            description: None,
            sub_description: None,
            content_type: "".to_string(),
            file_name: "".to_string(),
            type_ids: vec![],
            ..Default::default()
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
                    flashcard.sub_description = Some(field.text().await.unwrap());
                }
                Some("image_data") => {
                    flashcard.content_type = field.content_type().unwrap_or_default().to_string();
                    flashcard.file_name = field.file_name().unwrap_or_default().to_string();
                    let bytes_data = field.bytes().await;
                    match bytes_data {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                flashcard.image_data = Some(bytes.to_vec());
                            }
                        }
                        Err(_) => {}
                    }
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

    pub async fn update_flashcard<T: AppStateTrait>(
        State(_state): State<T>,
        Path(id): Path<i32>,
        mut multipart: Multipart,
    ) -> Json<Option<i32>> {
        let mut flashcard = FlashcardUpdationDto {
            name: None,
            description: None,
            sub_description: None,
            content_type: None,
            file_name: None,
            type_ids: None,
            image_data: None,
        };
        while let Some(field) = multipart.next_field().await.unwrap() {
            match field.name() {
                Some("name") => {
                    flashcard.name = Some(field.text().await.unwrap());
                }
                Some("description") => {
                    flashcard.description = Some(field.text().await.unwrap());
                }
                Some("sub_description") => {
                    flashcard.sub_description = Some(field.text().await.unwrap());
                }
                Some("image_data") => {
                    flashcard.content_type =
                        Some(field.content_type().unwrap_or_default().to_string());
                    flashcard.file_name = Some(field.file_name().unwrap_or_default().to_string());
                    let bytes_data = field.bytes().await;
                    match bytes_data {
                        Ok(bytes) => {
                            if bytes.len() > 0 {
                                flashcard.image_data = Some(bytes.to_vec());
                            }
                        }
                        Err(_) => {}
                    }
                }
                _ => {}
            }
        }

        let result = _state
            .flashcard_usecase()
            .update_flashcard(id, flashcard)
            .await;
        return match result {
            None => Json(None),
            Some(i) => Json(Some(i)),
        };
    }
}

pub struct FlashcardHandler {}
