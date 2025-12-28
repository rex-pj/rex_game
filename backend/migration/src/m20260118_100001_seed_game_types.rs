use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::enums::game_type::GameType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = chrono::Utc::now();

        // Seed default game types
        let insert = Query::insert()
            .into_table(GameType::Table)
            .columns([
                GameType::Code,
                GameType::Name,
                GameType::Description,
                GameType::Icon,
                GameType::IsActived,
                GameType::CreatedDate,
                GameType::UpdatedDate,
            ])
            .values_panic([
                "memory_match".into(),
                "Memory Match".into(),
                "Match pairs of flashcards by flipping them over".into(),
                "fa-solid fa-clone".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "quiz".into(),
                "Quiz Mode".into(),
                "Answer questions about flashcard content".into(),
                "fa-solid fa-question-circle".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "speed_match".into(),
                "Speed Match".into(),
                "Match flashcards as fast as possible".into(),
                "fa-solid fa-bolt".into(),
                true.into(),
                now.into(),
                now.into(),
            ])
            .values_panic([
                "spelling".into(),
                "Spelling Game".into(),
                "Spell the word shown in the flashcard image".into(),
                "fa-solid fa-spell-check".into(),
                false.into(),
                now.into(),
                now.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(GameType::Table)
            .and_where(Expr::col(GameType::Code).is_in([
                "memory_match",
                "quiz",
                "speed_match",
                "spelling",
            ]))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
