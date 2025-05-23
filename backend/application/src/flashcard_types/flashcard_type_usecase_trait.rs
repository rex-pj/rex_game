use std::future::Future;

use crate::{errors::application_error::ApplicationError, page_list_dto::PageListDto};

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
    ) -> impl Future<Output = Result<PageListDto<FlashcardTypeDto>, ApplicationError>>;
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
    ) -> impl Future<Output = Option<FlashcardTypeDto>>;
    fn delete_flashcard_type_by_id(&self, id: i32) -> impl Future<Output = Option<u64>>;
}
