use sea_orm::EnumIter;
use sea_orm_migration::prelude::*;

#[derive(Iden, EnumIter)]
pub enum UserStatus {
    #[iden = "Pending"]
    Pending,
    #[iden = "Actived"]
    Actived,
    #[iden = "Deleted"]
    Deleted,
}
