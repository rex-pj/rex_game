pub mod flashcard;
pub mod flashcard_file;
pub mod flashcard_type;
pub mod flashcard_type_relation;

pub mod prelude {
    pub use super::flashcard::Entity as Flashcard;
    pub use super::flashcard_file::Entity as FlashcardFile;
    pub use super::flashcard_type::Entity as FlashcardType;
    pub use super::flashcard_type_relation::Entity as FlashcardTypeRelation;
}

pub use flashcard::Entity as Flashcard;
pub use flashcard_file::Entity as FlashcardFile;
pub use flashcard_type::Entity as FlashcardType;
pub use flashcard_type_relation::Entity as FlashcardTypeRelation;
