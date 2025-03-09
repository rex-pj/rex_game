use sea_orm_migration::{async_trait::async_trait, prelude::*, sea_orm::Iterable};

use crate::enums::{user::User, user_status::UserStatus};

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
