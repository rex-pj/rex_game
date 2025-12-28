use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use crate::enums::{
    flashcard_type::FlashcardType,
    game_session::GameSession,
    game_type::GameType,
    user::User,
    user_achievement::{Achievement, UserAchievement},
    user_stats::UserStats,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create game_type table
        manager
            .create_table(
                Table::create()
                    .table(GameType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GameType::Code)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(GameType::Name).string().not_null())
                    .col(ColumnDef::new(GameType::Description).string())
                    .col(ColumnDef::new(GameType::Icon).string())
                    .col(
                        ColumnDef::new(GameType::IsActived)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(GameType::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GameType::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create game_session table
        manager
            .create_table(
                Table::create()
                    .table(GameSession::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameSession::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameSession::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_session-user_id")
                            .from(GameSession::Table, GameSession::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(GameSession::GameTypeId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_session-game_type_id")
                            .from(GameSession::Table, GameSession::GameTypeId)
                            .to(GameType::Table, GameType::Id),
                    )
                    .col(ColumnDef::new(GameSession::FlashcardTypeId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game_session-flashcard_type_id")
                            .from(GameSession::Table, GameSession::FlashcardTypeId)
                            .to(FlashcardType::Table, FlashcardType::Id),
                    )
                    .col(
                        ColumnDef::new(GameSession::Score)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(GameSession::MaxScore).integer())
                    .col(ColumnDef::new(GameSession::Accuracy).decimal_len(5, 2))
                    .col(
                        ColumnDef::new(GameSession::TimeSpentSeconds)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GameSession::CardsPlayed)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GameSession::CorrectAnswers)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GameSession::WrongAnswers)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GameSession::ComboMax)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(GameSession::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GameSession::CompletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(GameSession::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create achievement table
        manager
            .create_table(
                Table::create()
                    .table(Achievement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Achievement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Achievement::Code)
                            .string_len(50)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Achievement::Name).string().not_null())
                    .col(ColumnDef::new(Achievement::Description).string())
                    .col(ColumnDef::new(Achievement::Icon).string())
                    .col(
                        ColumnDef::new(Achievement::Points)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Achievement::Category).string_len(50))
                    .col(
                        ColumnDef::new(Achievement::IsActived)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Achievement::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Achievement::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Create user_achievement table
        manager
            .create_table(
                Table::create()
                    .table(UserAchievement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserAchievement::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserAchievement::UserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_achievement-user_id")
                            .from(UserAchievement::Table, UserAchievement::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(UserAchievement::AchievementId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_achievement-achievement_id")
                            .from(UserAchievement::Table, UserAchievement::AchievementId)
                            .to(Achievement::Table, Achievement::Id),
                    )
                    .col(
                        ColumnDef::new(UserAchievement::UnlockedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add unique constraint for user_achievement
        manager
            .create_index(
                Index::create()
                    .name("idx-user_achievement-unique")
                    .table(UserAchievement::Table)
                    .col(UserAchievement::UserId)
                    .col(UserAchievement::AchievementId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // 5. Create user_stats table
        manager
            .create_table(
                Table::create()
                    .table(UserStats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserStats::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserStats::UserId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user_stats-user_id")
                            .from(UserStats::Table, UserStats::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(UserStats::TotalScore)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::TotalGamesPlayed)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::TotalTimePlayedSeconds)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::BestScore)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::BestCombo)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::AverageAccuracy)
                            .decimal_len(5, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::CurrentStreak)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(UserStats::BestStreak)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(UserStats::LastPlayedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(UserStats::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for better query performance
        manager
            .create_index(
                Index::create()
                    .name("idx-game_session-user_id")
                    .table(GameSession::Table)
                    .col(GameSession::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-game_session-score")
                    .table(GameSession::Table)
                    .col(GameSession::Score)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-user_stats-total_score")
                    .table(UserStats::Table)
                    .col(UserStats::TotalScore)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop indexes
        manager
            .drop_index(
                Index::drop()
                    .name("idx-user_stats-total_score")
                    .table(UserStats::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-game_session-score")
                    .table(GameSession::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-game_session-user_id")
                    .table(GameSession::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx-user_achievement-unique")
                    .table(UserAchievement::Table)
                    .to_owned(),
            )
            .await?;

        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(UserStats::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserAchievement::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Achievement::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GameSession::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GameType::Table).to_owned())
            .await?;

        Ok(())
    }
}
