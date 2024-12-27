use rex_game_domain::flashcards::{
    flashcard::{CreateFlashcardError, CreateFlashcardRequest, Flashcard},
    t_flashcard_repository::TFlashcardRepository,
};
use std::{future::Future, pin::Pin};

use super::{flashcard_dto::FlashcardDto, t_flashcard_usecase::TFlashcardUseCase};

#[derive(Clone)]
pub struct FlashcardUseCase<TF>
where
    TF: TFlashcardRepository,
{
    _flashcard_repository: TF,
}

impl<TF: TFlashcardRepository> FlashcardUseCase<TF> {
    pub fn new(flashcard_repository: TF) -> Self {
        Self {
            _flashcard_repository: flashcard_repository,
        }
    }
}

impl<TF: TFlashcardRepository> TFlashcardUseCase for FlashcardUseCase<TF> {
    fn create_flashcard<'a>(
        &'a self,
        req: &'a CreateFlashcardRequest,
    ) -> Pin<Box<dyn Future<Output = Result<Flashcard, CreateFlashcardError>> + Send + 'a>> {
        Box::pin(async move {
            if req.description == "test" {
                let flashcard = Flashcard::new("Hello", "Ajinomoto", "Olala hahahahha");
                return Ok(flashcard);
            }

            let error = CreateFlashcardError::InvalidData(String::from("Loi roi"));
            Err(error)
        })
    }

    fn get_flashcard<'a>(&'a self) -> Option<FlashcardDto> {
        let existing = self._flashcard_repository.get_flashcard();
        match existing {
            None => None,
            Some(i) => Some(FlashcardDto {
                id: i.id,
                created_date: i.created_date,
                updated_date: i.updated_date,
                description: i.description,
                name: i.name,
                sub_description: i.sub_description,
            }),
        }
    }
}
