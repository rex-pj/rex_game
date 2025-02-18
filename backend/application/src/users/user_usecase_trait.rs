use std::future::Future;

use sea_orm::DbErr;

use super::{user_creation_dto::UserCreationDto, user_details_dto::UserDetailsDto};

pub trait UserUseCaseTrait {
    fn get_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> impl Future<Output = Result<UserDetailsDto, DbErr>>;

    fn create_user<'a>(&'a self, user_req: UserCreationDto) -> impl Future<Output = Option<i32>>;
}
