use sea_orm_migration::prelude::*;

use crate::enums::{game_type::GameType, user::User, user_game_progress::UserGameProgress};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create user_game_progress table
        manager
            .create_table(
                Table::create()
                    .table(UserGameProgress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserGameProgress::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::GameTypeId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::CurrentLevel)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::HighestLevel)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::TotalScore)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::LastPlayedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserGameProgress::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_game_progress_user")
                            .from(UserGameProgress::Table, UserGameProgress::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_game_progress_game_type")
                            .from(UserGameProgress::Table, UserGameProgress::GameTypeId)
                            .to(GameType::Table, GameType::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique index on (user_id, game_type_id)
        manager
            .create_index(
                Index::create()
                    .name("idx_user_game_progress_unique")
                    .table(UserGameProgress::Table)
                    .col(UserGameProgress::UserId)
                    .col(UserGameProgress::GameTypeId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGameProgress::Table).to_owned())
            .await
    }
}
