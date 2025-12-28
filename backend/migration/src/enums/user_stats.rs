use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum UserStats {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "total_score")]
    TotalScore,
    #[sea_orm(iden = "total_games_played")]
    TotalGamesPlayed,
    #[sea_orm(iden = "total_time_played_seconds")]
    TotalTimePlayedSeconds,
    #[sea_orm(iden = "best_score")]
    BestScore,
    #[sea_orm(iden = "best_combo")]
    BestCombo,
    #[sea_orm(iden = "average_accuracy")]
    AverageAccuracy,
    #[sea_orm(iden = "current_streak")]
    CurrentStreak,
    #[sea_orm(iden = "best_streak")]
    BestStreak,
    #[sea_orm(iden = "last_played_at")]
    LastPlayedAt,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}
