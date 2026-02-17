use sea_orm_migration::prelude::*;

use crate::enums::{
    flashcard::Flashcard, game_type::GameType, game_type_flashcard::GameTypeFlashcard,
    user::User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GameTypeFlashcard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameTypeFlashcard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GameTypeFlashcard::GameTypeId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GameTypeFlashcard::FlashcardId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GameTypeFlashcard::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GameTypeFlashcard::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GameTypeFlashcard::CreatedById).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_type_flashcard-created-by-id")
                            .from(GameTypeFlashcard::Table, GameTypeFlashcard::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(GameTypeFlashcard::UpdatedById).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_type_flashcard-updated-by-id")
                            .from(GameTypeFlashcard::Table, GameTypeFlashcard::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_type_flashcard-game_type")
                            .from(
                                GameTypeFlashcard::Table,
                                GameTypeFlashcard::GameTypeId,
                            )
                            .to(GameType::Table, GameType::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_type_flashcard-flashcard")
                            .from(
                                GameTypeFlashcard::Table,
                                GameTypeFlashcard::FlashcardId,
                            )
                            .to(Flashcard::Table, Flashcard::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-game_type_flashcard-unique")
                    .table(GameTypeFlashcard::Table)
                    .col(GameTypeFlashcard::GameTypeId)
                    .col(GameTypeFlashcard::FlashcardId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameTypeFlashcard::Table).to_owned())
            .await?;

        Ok(())
    }
}
