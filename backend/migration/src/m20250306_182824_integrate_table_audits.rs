use sea_orm_migration::prelude::*;

use crate::enums::{
    flashcard::Flashcard, flashcard_file::FlashcardType, flashcard_type::FlashcardFile,
    flashcard_type_relation::FlashcardTypeRelation, user::User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardType::Table)
                    .add_column(ColumnDef::new(FlashcardType::CreatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-type-created-by-id")
                            .from_tbl(FlashcardType::Table)
                            .from_col(FlashcardType::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .add_column(ColumnDef::new(FlashcardType::UpdatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-type-updated-by-id")
                            .from_tbl(FlashcardType::Table)
                            .from_col(FlashcardType::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Flashcard::Table)
                    .add_column(ColumnDef::new(Flashcard::CreatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-created-by-id")
                            .from_tbl(Flashcard::Table)
                            .from_col(Flashcard::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .add_column(ColumnDef::new(Flashcard::UpdatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-updated-by-id")
                            .from_tbl(Flashcard::Table)
                            .from_col(Flashcard::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardFile::Table)
                    .add_column(ColumnDef::new(FlashcardFile::CreatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-file-created-by-id")
                            .from_tbl(FlashcardFile::Table)
                            .from_col(FlashcardFile::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .add_column(ColumnDef::new(FlashcardFile::UpdatedById).integer().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-file-updated-by-id")
                            .from_tbl(FlashcardFile::Table)
                            .from_col(FlashcardFile::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardTypeRelation::Table)
                    .add_column(
                        ColumnDef::new(FlashcardTypeRelation::CreatedById)
                            .integer()
                            .null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-type-relation-created-by-id")
                            .from_tbl(FlashcardTypeRelation::Table)
                            .from_col(FlashcardTypeRelation::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .add_column(
                        ColumnDef::new(FlashcardTypeRelation::UpdatedById)
                            .integer()
                            .null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-flashcard-type-relation-updated-by-id")
                            .from_tbl(FlashcardTypeRelation::Table)
                            .from_col(FlashcardTypeRelation::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardType::Table)
                    .drop_column(FlashcardType::CreatedById)
                    .drop_column(FlashcardType::UpdatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Flashcard::Table)
                    .drop_column(Flashcard::CreatedById)
                    .drop_column(Flashcard::UpdatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardFile::Table)
                    .drop_column(FlashcardFile::CreatedById)
                    .drop_column(FlashcardFile::UpdatedById)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FlashcardTypeRelation::Table)
                    .drop_column(FlashcardTypeRelation::CreatedById)
                    .drop_column(FlashcardTypeRelation::UpdatedById)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
