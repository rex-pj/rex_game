use sea_orm_migration::{prelude::*, schema::*, sea_orm::Iterable};

use crate::enums::{
    user::User,
    user_token::{UserToken, UserTokenPurpose},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserToken::Table)
                    .if_not_exists()
                    .col(pk_auto(UserToken::Id))
                    .col(ColumnDef::new(UserToken::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-token-user-id")
                            .from(UserToken::Table, UserToken::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(UserToken::Token).string().not_null())
                    .col(ColumnDef::new(UserToken::Expiration).unsigned().not_null())
                    .col(ColumnDef::new(UserToken::CreatedById).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-token-created-by-id")
                            .from(UserToken::Table, UserToken::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(UserToken::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserToken::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserToken::UpdatedById).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-token-updated-by-id")
                            .from(UserToken::Table, UserToken::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(UserToken::IsActived).boolean().not_null())
                    .col(
                        ColumnDef::new(UserToken::Purpose)
                            .enumeration(Alias::new("UserTokenPurpose"), UserTokenPurpose::iter())
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserToken::Table).to_owned())
            .await
    }
}
