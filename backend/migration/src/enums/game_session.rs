use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum GameSession {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "game_type_id")]
    GameTypeId,
    #[sea_orm(iden = "flashcard_type_id")]
    FlashcardTypeId,
    #[sea_orm(iden = "score")]
    Score,
    #[sea_orm(iden = "max_score")]
    MaxScore,
    #[sea_orm(iden = "accuracy")]
    Accuracy,
    #[sea_orm(iden = "time_spent_seconds")]
    TimeSpentSeconds,
    #[sea_orm(iden = "cards_played")]
    CardsPlayed,
    #[sea_orm(iden = "correct_answers")]
    CorrectAnswers,
    #[sea_orm(iden = "wrong_answers")]
    WrongAnswers,
    #[sea_orm(iden = "combo_max")]
    ComboMax,
    #[sea_orm(iden = "started_at")]
    StartedAt,
    #[sea_orm(iden = "completed_at")]
    CompletedAt,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
}
