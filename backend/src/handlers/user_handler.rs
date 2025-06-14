use axum::http::HeaderMap;
use axum::{extract::State, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        application_user_dto::ApplicationUserDto,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    users::{loggedin_user_dto::LoggedInUserDto, user_statuses::UserStatuses},
};

use crate::{app_state::AppStateTrait, view_models::users::signup_request::SignupRequest};

impl UserHandler {
    pub async fn create_user<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<SignupRequest>>,
    ) -> Result<Json<i32>, StatusCode> {
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
        let signup_result = _state
            .identity_user_usecase()
            .create_user(new_user, &req.password)
            .await;
        match signup_result {
            Ok(created) => Ok(Json(created.id)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_current_user<T: AppStateTrait>(
        headers: HeaderMap,
        State(_state): State<T>,
    ) -> Result<Json<LoggedInUserDto>, StatusCode> {
        let access_token_header = match headers.get("authorization") {
            Some(authorization) => authorization,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = match access_token_header.to_str() {
            Ok(authorization) => authorization,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = access_token.strip_prefix("Bearer ").unwrap();
        let logged_in_user_result = _state
            .identity_user_usecase()
            .get_logged_in_user(access_token)
            .await;

        match logged_in_user_result {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

pub struct UserHandler {}
