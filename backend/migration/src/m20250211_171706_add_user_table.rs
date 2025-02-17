use sea_orm::EnumIter;
use sea_orm_migration::{async_trait::async_trait, prelude::*, sea_orm::Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::DisplayName).string())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::SecurityStamp).string().not_null())
                    .col(ColumnDef::new(User::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-created-by")
                            .from(User::Table, User::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(User::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::UpdatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-updated-by")
                            .from(User::Table, User::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(User::StatusId)
                            .enumeration(Alias::new("UserStatus"), UserStatus::iter())
                            .integer()
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
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "display_name")]
    DisplayName,
    #[sea_orm(iden = "password_hash")]
    PasswordHash,
    #[sea_orm(iden = "security_stamp")]
    SecurityStamp,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "status_id")]
    StatusId,
}

#[derive(Iden, EnumIter)]
pub enum UserStatus {
    #[iden = "Pending"]
    Pending,
    #[iden = "Actived"]
    Actived,
    #[iden = "Deleted"]
    Deleted,
}
