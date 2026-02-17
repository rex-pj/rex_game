use sea_orm_migration::sea_orm::{self, DeriveIden};

#[derive(DeriveIden)]
pub enum MailTemplate {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "subject")]
    Subject,
    #[sea_orm(iden = "body")]
    Body,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "is_actived")]
    IsActived,
    #[sea_orm(iden = "is_enabled")]
    IsEnabled,
}
