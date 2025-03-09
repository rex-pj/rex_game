use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        application_user_dto::ApplicationUserDto,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    users::{roles::ROLE_ADMIN, user_statuses::UserStatuses, user_usecase_trait::UserUseCaseTrait},
};

use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};

impl SetupHandler {
    pub async fn setup<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> Result<Json<bool>, StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };
        let new_user = ApplicationUserDto {
            email: req.email,
            name: req.name,
            display_name: req.display_name,
            status_id: UserStatuses::Actived as i32,
            ..Default::default()
        };
        let signup_result = match _state
            .identity_user_usecase()
            .create_user(new_user, &req.password)
            .await
        {
            Ok(created) => created,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        match _state
            .user_usecase()
            .assign_role(signup_result.id, ROLE_ADMIN)
            .await
        {
            Ok(_) => Ok(Json(true)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct SetupHandler {}
