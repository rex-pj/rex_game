use std::future::Future;

use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};

use super::{
    flashcard_type_creation_dto::FlashcardTypeCreationDto, flashcard_type_dto::FlashcardTypeDto,
    flashcard_type_updation_dto::FlashcardTypeUpdationDto,
};

pub trait FlashcardTypeUseCaseTrait {
    fn get_flashcard_types<'a>(
        &'a self,
        name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardTypeDto>, ApplicationError>>;
    fn get_flashcard_type_by_id<'a>(
        &'a self,
        id: i32,
    ) -> impl Future<Output = Option<FlashcardTypeDto>>;
    fn create_flashcard_type<'a>(
        &'a self,
        flashcard: FlashcardTypeCreationDto,
    ) -> impl Future<Output = Option<i32>>;

    fn update_flashcard_type<'a>(
        &'a self,
        id: i32,
        flashcard: FlashcardTypeUpdationDto,
    ) -> impl Future<Output = Option<bool>>;
    fn delete_flashcard_type_by_id(&self, id: i32) -> impl Future<Output = Option<u64>>;
    fn get_flashcard_type_by_flashcard_id<'a>(
        &'a self,
        flashcard_id: i32,
    ) -> impl Future<Output = Option<Vec<FlashcardTypeDto>>>;
}
