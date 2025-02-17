use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        application_user_dto::ApplicationUserDto,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    users::user_statuses::UserStatuses,
};

use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};

impl UserHandler {
    pub async fn create_user<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
        match payload {
            Some(req) => {
                let new_user = ApplicationUserDto {
                    email: req.email,
                    name: req.name,
                    display_name: req.display_name,
                    status_id: UserStatuses::Actived as i32,
                    ..Default::default()
                };
                let signup_result = _state
                    .identity_user_usecase()
                    .create_user(new_user, &req.password)
                    .await;
                match signup_result {
                    Ok(created) => Ok(Json(created.id)),
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
            None => Err(StatusCode::BAD_REQUEST),
        }
    }
}

pub struct UserHandler {}
