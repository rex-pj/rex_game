use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

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
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Flashcard::Table).to_owned())
            .await
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
}
