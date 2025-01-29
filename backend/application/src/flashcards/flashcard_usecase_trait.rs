use std::future::Future;

use super::{
    flashcard_creation_dto::FlashcardCreationDto, flashcard_dto::FlashcardDto,
    flashcard_updation_dto::FlashcardUpdationDto,
};

pub trait FlashcardUseCaseTrait {
    fn get_flashcards<'a>(
        &'a self,
        type_name: Option<String>,
        page: u64,
        page_size: u64,
    ) -> impl Future<Output = Option<Vec<FlashcardDto>>>;
    fn get_flashcard_by_id<'a>(&'a self, id: i32) -> impl Future<Output = Option<FlashcardDto>>;
    fn create_flashcard<'a>(
        &'a self,
        flashcard: FlashcardCreationDto,
    ) -> impl Future<Output = Option<i32>>;
    fn get_image_by_file_id<'a>(&'a self, file_id: i32) -> impl Future<Output = Option<Vec<u8>>>;

    fn update_flashcard<'a>(
        &'a self,
        id: i32,
        flashcard_req: FlashcardUpdationDto,
    ) -> impl Future<Output = Option<i32>>;
}
