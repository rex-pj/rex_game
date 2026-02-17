use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum User {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "display_name")]
    DisplayName,
    #[sea_orm(iden = "password_hash")]
    PasswordHash,
    #[sea_orm(iden = "security_stamp")]
    SecurityStamp,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "created_on")]
    CreatedOn,
    #[sea_orm(iden = "created_by_id")]
    CreatedById,
    #[sea_orm(iden = "updated_on")]
    UpdatedOn,
    #[sea_orm(iden = "updated_by_id")]
    UpdatedById,
    #[sea_orm(iden = "status_id")]
    StatusId,
}
