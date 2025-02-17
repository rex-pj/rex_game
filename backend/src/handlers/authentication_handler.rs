use axum::{extract::State, response::Result, Json};
use hyper::StatusCode;
use rex_game_application::{
    identities::identity_login_usecase_trait::IdentityLoginUseCaseTrait,
    users::user_login_parameter::UserLoginParameter,
};

use crate::{app_state::AppStateTrait, view_models::users::login_result::LoginResult};

impl AuthenticationHandler {
    pub async fn login<T: AppStateTrait>(
        State(_state): State<T>,
        Json(payload): Json<Option<UserLoginParameter>>,
    ) -> Result<Json<LoginResult>, StatusCode> {
        match payload {
            Some(req) => {
                let login_result = _state
                    .identity_login_usecase()
                    .password_login(&req.email, &req.password)
                    .await;

                match login_result {
                    Ok(result) => Ok(Json(LoginResult {
                        display_name: result.display_name,
                        user_email: result.user_email,
                        refresh_token: result.refresh_token,
                        token: result.token,
                        user_id: result.user_id,
                    })),
                    Err(_) => Err(StatusCode::UNAUTHORIZED),
                }
            }
            None => Err(StatusCode::BAD_REQUEST),
        }
    }
}

pub struct AuthenticationHandler {}
