use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum GameType {
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
    #[sea_orm(iden = "is_actived")]
    IsActived,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
}
