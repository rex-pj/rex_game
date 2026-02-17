use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum UserAchievement {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "achievement_id")]
    AchievementId,
    #[sea_orm(iden = "unlocked_at")]
    UnlockedAt,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
}

#[derive(DeriveIden)]
pub enum Achievement {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "code")]
    Code,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "icon")]
    Icon,
    #[sea_orm(iden = "points")]
    Points,
    #[sea_orm(iden = "category")]
    Category,
    #[sea_orm(iden = "is_actived")]
    IsActived,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
}
