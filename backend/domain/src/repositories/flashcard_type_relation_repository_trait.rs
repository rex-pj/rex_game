use std::future::Future;

use crate::{
    errors::domain_error::DomainError,
    models::flashcard_type_relation_model::FlashcardTypeRelationModel,
};

pub trait FlashcardTypeRelationRepositoryTrait {
    fn create(
        &self,
        flashcard_type_relations_req: Vec<FlashcardTypeRelationModel>,
    ) -> impl Future<Output = Result<i32, DomainError>>;

    fn get_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> impl Future<Output = Result<Vec<FlashcardTypeRelationModel>, DomainError>>;

    fn delete_by_ids(&self, ids: Vec<i32>) -> impl Future<Output = Result<u64, DomainError>>;
    fn delete_by_flashcard_id(
        &self,
        flashcard_id: i32,
    ) -> impl Future<Output = Result<u64, DomainError>>;
}
