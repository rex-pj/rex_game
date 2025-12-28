use sea_orm_migration::sea_orm::{self, DeriveIden};

#[derive(DeriveIden)]
pub enum SystemSettings {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "key")]
    Key,
    #[sea_orm(iden = "value")]
    Value,
    #[sea_orm(iden = "description")]
    Description,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "created_date")]
    CreatedDate,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "updated_date")]
    UpdatedDate,
    #[sea_orm(iden = "is_actived")]
    IsActived,
}
