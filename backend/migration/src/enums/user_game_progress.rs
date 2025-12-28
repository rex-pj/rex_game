use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum UserGameProgress {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "game_type_id")]
    GameTypeId,
    #[sea_orm(iden = "current_level")]
    CurrentLevel,
    #[sea_orm(iden = "highest_level")]
    HighestLevel,
    #[sea_orm(iden = "total_score")]
    TotalScore,
    #[sea_orm(iden = "last_played_at")]
    LastPlayedAt,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}
