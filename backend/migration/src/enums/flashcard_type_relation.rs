use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum FlashcardTypeRelation {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "flashcard_id")]
    FlashcardId,
    #[sea_orm(iden = "flashcard_type_id")]
    FlashcardTypeId,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}
