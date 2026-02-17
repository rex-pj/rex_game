use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum GameTypeFlashcard {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "game_type_id")]
    GameTypeId,
    #[sea_orm(iden = "flashcard_id")]
    FlashcardId,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
}
