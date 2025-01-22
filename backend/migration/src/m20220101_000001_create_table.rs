use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(FlashcardType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FlashcardType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FlashcardType::Name).string().not_null())
                    .col(ColumnDef::new(FlashcardType::Description).string())
                    .col(
                        ColumnDef::new(FlashcardType::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardType::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Flashcard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Flashcard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Flashcard::Name).string().not_null())
                    .col(ColumnDef::new(Flashcard::Description).string())
                    .col(
                        ColumnDef::new(Flashcard::SubDescription)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Flashcard::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Flashcard::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Flashcard::ImageData).binary().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(FlashcardTypeRelation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::FlashcardId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::FlashcardTypeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-flashcard_type_relation-flashcard")
                            .from(
                                FlashcardTypeRelation::Table,
                                FlashcardTypeRelation::FlashcardId,
                            )
                            .to(Flashcard::Table, Flashcard::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-flashcard_type_relation-flashcard_type")
                            .from(
                                FlashcardTypeRelation::Table,
                                FlashcardTypeRelation::FlashcardTypeId,
                            )
                            .to(FlashcardType::Table, FlashcardType::Id),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Flashcard::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FlashcardType::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FlashcardTypeRelation::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Flashcard {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "sub_description")]
    SubDescription,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
    #[sea_orm(iden = "image_data")]
    ImageData,
}

#[derive(DeriveIden)]
enum FlashcardType {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}

#[derive(DeriveIden)]
enum FlashcardTypeRelation {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "flashcard_id")]
    FlashcardId,
    #[sea_orm(iden = "flashcard_type_id")]
    FlashcardTypeId,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}
