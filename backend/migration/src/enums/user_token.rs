use sea_orm::EnumIter;
use sea_orm_migration::prelude::*;

#[derive(Iden, EnumIter)]
pub enum UserTokenPurpose {
    #[iden = "Login"]
    Login = 1,
    #[iden = "Refresh"]
    Refresh = 2,
    #[iden = "PasswordReset"]
    PasswordReset = 3,
    #[iden = "EmailVerification"]
    EmailVerification = 4,
}

#[derive(DeriveIden)]
pub enum UserToken {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "user_id")]
    UserId,
    #[sea_orm(iden = "token")]
    Token,
    #[sea_orm(iden = "expiration")]
    Expiration,
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
    #[sea_orm(iden = "purpose")]
    Purpose,
}
