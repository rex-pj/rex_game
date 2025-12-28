use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_file_dto::FlashcardFileDto, flashcard_updation_dto::FlashcardUpdationDto,
};
use rex_game_shared_kernel::{domain::errors::domain_error::DomainError, domain::models::page_list_model::PageListModel};
use std::future::Future;

pub trait FlashcardUseCaseTrait {
    fn get_paged_list<'a>(
        &'a self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Result<PageListModel<FlashcardDto>, DomainError>>;
    fn get_flashcard_by_id<'a>(&'a self, id: i32) -> impl Future<Output = Option<FlashcardDto>>;
    fn create_flashcard<'a>(
        &'a self,
        flashcard: FlashcardCreationDto,
    ) -> impl Future<Output = Result<i32, DomainError>>;
    fn get_image_by_file_id<'a>(
        &'a self,
        file_id: i32,
    ) -> impl Future<Output = Result<FlashcardFileDto, DomainError>>;

    fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> impl Future<Output = Result<bool, DomainError>>;
    fn delete_flashcard_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<u64, DomainError>>;
}
