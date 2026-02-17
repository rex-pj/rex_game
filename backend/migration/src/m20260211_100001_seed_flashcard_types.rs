use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::enums::flashcard_type::FlashcardType;

#[derive(DeriveMigrationName)]
pub struct Migration;

const SEED_NAMES: [&str; 5] = ["Animals", "Fruits", "Colors", "Numbers", "Shapes"];

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = chrono::Utc::now();

        let seeds: Vec<(&str, &str)> = vec![
            ("Animals", "Flashcards about animals"),
            ("Fruits", "Flashcards about fruits"),
            ("Colors", "Flashcards about colors"),
            ("Numbers", "Flashcards about numbers and counting"),
            ("Shapes", "Flashcards about shapes"),
        ];

        let mut insert = Query::insert()
            .into_table(FlashcardType::Table)
            .columns([
                FlashcardType::Name,
                FlashcardType::Description,
                FlashcardType::IsActived,
                FlashcardType::CreatedOn,
                FlashcardType::UpdatedOn,
            ])
            .to_owned();

        for (name, description) in seeds {
            insert.values_panic([
                name.into(),
                description.into(),
                true.into(),
                now.into(),
                now.into(),
            ]);
        }

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(FlashcardType::Table)
            .and_where(Expr::col(FlashcardType::Name).is_in(SEED_NAMES))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
