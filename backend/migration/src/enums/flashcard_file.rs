use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum FlashcardFile {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "file_name")]
    FileName,
    #[sea_orm(iden = "content_type")]
    ContentType,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "data")]
    Data,
    #[sea_orm(iden = "is_actived")]
    IsActived,
}
