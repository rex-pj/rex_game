use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_file_dto::FlashcardFileDto, flashcard_updation_dto::FlashcardUpdationDto,
};
use rex_game_shared::{domain::models::page_list_model::PageListModel, ApplicationError};
use std::future::Future;

pub trait FlashcardUseCaseTrait {
    fn get_paged_list<'a>(
        &'a self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardDto>, ApplicationError>>;
    fn get_flashcard_by_id<'a>(&'a self, id: i32) -> impl Future<Output = Option<FlashcardDto>>;
    fn create_flashcard<'a>(
        &'a self,
        flashcard: FlashcardCreationDto,
    ) -> impl Future<Output = Result<i32, ApplicationError>>;
    fn get_image_by_file_id<'a>(
        &'a self,
        file_id: i32,
    ) -> impl Future<Output = Result<FlashcardFileDto, ApplicationError>>;

    fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> impl Future<Output = Result<bool, ApplicationError>>;
    fn delete_flashcard_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<u64, ApplicationError>>;
    fn toggle_flashcard_active(
        &self,
        id: i32,
        updated_by_id: i32,
    ) -> impl Future<Output = Result<bool, ApplicationError>>;
}
