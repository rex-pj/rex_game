use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Flashcard {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "sub_description")]
    SubDescription,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "file_id")]
    FileId,
    #[sea_orm(iden = "is_actived")]
    IsActived,
}
