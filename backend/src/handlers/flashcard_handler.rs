use crate::{
    app_state::AppStateTrait, helpers::http_helper::HttpHelper,
    middlewares::authenticate_middleware::CurrentUser,
};
use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::{Response, Result},
    Extension, Json,
};
use rex_game_application::flashcards::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_updation_dto::FlashcardUpdationDto, flashcard_usecase_trait::FlashcardUseCaseTrait,
};
use serde::Deserialize;
use std::sync::Arc;

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
    ) -> Result<Json<FlashcardDto>, StatusCode> {
        let flashcard = _state.flashcard_usecase().get_flashcard_by_id(id).await;
        return match flashcard {
            None => return Err(StatusCode::NOT_FOUND),
            Some(i) => Ok(Json(i)),
        };
    }

    pub async fn get_flashcard_image<T: AppStateTrait>(
        Path(file_id): Path<i32>,
        State(_state): State<T>,
    ) -> Result<Response<Body>, StatusCode> {
        let flashcard_file = match _state
            .flashcard_usecase()
            .get_image_by_file_id(file_id)
            .await
        {
            Ok(file) => file,
            Err(_) => return Err(StatusCode::NOT_FOUND),
        };

        match HttpHelper::build_file_respone(flashcard_file.data, &flashcard_file.content_type) {
            Ok(response) => Ok(response),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn create_flashcard<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        mut multipart: Multipart,
    ) -> Result<Json<i32>, StatusCode> {
        let mut flashcard = FlashcardCreationDto {
            name: "".to_string(),
            description: None,
            sub_description: None,
            content_type: "".to_string(),
            file_name: "".to_string(),
            created_by_id: Some(current_user.id),
            updated_by_id: Some(current_user.id),
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
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Ok(i) => Ok(Json(i)),
        };
    }

    pub async fn update_flashcard<T: AppStateTrait>(
        Extension(current_user): Extension<Arc<CurrentUser>>,
        State(_state): State<T>,
        Path(id): Path<i32>,
        mut multipart: Multipart,
    ) -> Result<Json<bool>, StatusCode> {
        let mut flashcard = FlashcardUpdationDto {
            updated_by_id: Some(current_user.id),
            ..Default::default()
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
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            Ok(_) => Ok(Json(true)),
        };
    }
}

pub struct FlashcardHandler {}
