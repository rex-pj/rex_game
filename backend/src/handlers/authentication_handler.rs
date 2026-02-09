use crate::{
    app_state::AppState,
    validators::validation_helper::ValidationHelper,
    view_models::{
        authentications::user_login_request::UserLoginRequest, users::login_result::LoginResult,
        HandlerError, HandlerResult,
    },
};
use axum::{extract::State, http::HeaderMap, Json};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, SameSite},
    CookieJar,
};
use hyper::StatusCode;
use rex_game_identity::application::usecases::auth::IdentityAuthenticateUseCaseTrait;
use rex_game_shared::infrastructure::helpers::datetime_helper_trait::DateTimeHelperTrait;
use validator::{Validate, ValidationErrors};

impl AuthenticationHandler {
    pub async fn login(
        State(_state): State<AppState>,
        jar: CookieJar,
        Json(payload): Json<Option<UserLoginRequest>>,
    ) -> HandlerResult<(CookieJar, Json<LoginResult>)> {
        let req: UserLoginRequest = match payload {
            Some(req) => req,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid request payload".to_string(),
                    ..Default::default()
                })
            }
        };

        req.validate().map_err(|e: ValidationErrors| {
            let errors = ValidationHelper::new().flatten_errors(e);
            return HandlerError {
                status: StatusCode::BAD_REQUEST,
                message: "Validation error".to_string(),
                field_errors: Some(errors),
            };
        })?;

        let token_claims = match _state
            .usecases
            .identity_authenticate
            .password_login(&req.email, &req.password)
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Authentication failed".to_string(),
                    ..Default::default()
                })
            }
        };

        let mut cookie = Cookie::new("refresh_token", token_claims.refresh_token);
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(true);
        let refresh_token_expires = _state
            .helpers
            .date_time
            .timestamp_to_offset_date_time(token_claims.refresh_token_expiration);
        match refresh_token_expires {
            Ok(offset) => cookie.set_expires(Expiration::DateTime(offset)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid refresh token expiration".to_string(),
                    ..Default::default()
                })
            }
        }

        let access_token_expires = match _state
            .helpers
            .date_time
            .timestamp_to_utc_date_time(token_claims.expiration)
        {
            Ok(offset) => offset,
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid access token expiration".to_string(),
                    ..Default::default()
                })
            }
        };

        Ok((
            jar.add(cookie),
            Json(LoginResult {
                access_token: token_claims.access_token,
                expiration: access_token_expires,
            }),
        ))
    }

    pub async fn logout(
        State(_state): State<AppState>,
        jar: CookieJar,
    ) -> HandlerResult<(CookieJar, Json<bool>)> {
        let mut cookie = Cookie::new("refresh_token", "");
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(true);
        cookie.set_max_age(time::Duration::ZERO);
        Ok((jar.remove(cookie), Json(true)))
    }

    pub async fn refresh_access_token(
        headers: HeaderMap,
        State(_state): State<AppState>,
        jar: CookieJar,
    ) -> HandlerResult<(CookieJar, Json<LoginResult>)> {
        let access_token_header = match headers.get("authorization") {
            Some(authorization) => authorization,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Missing authorization header".to_string(),
                    ..Default::default()
                })
            }
        };

        let access_token = match access_token_header.to_str() {
            Ok(authorization) => authorization,
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid authorization header".to_string(),
                    ..Default::default()
                })
            }
        };

        let access_token = match access_token.strip_prefix("Bearer ") {
            Some(token) => token,
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid authorization format".to_string(),
                    ..Default::default()
                })
            }
        };
        let req_refresh_token = match jar.get("refresh_token") {
            Some(refresh_token) => refresh_token.value().to_string(),
            None => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Missing refresh token cookie".to_string(),
                    ..Default::default()
                })
            }
        };
        let token_claims = match _state
            .usecases
            .identity_authenticate
            .refresh_access_token(&access_token, &req_refresh_token)
            .await
        {
            Ok(result) => result,
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Failed to refresh access token".to_string(),
                    ..Default::default()
                })
            }
        };

        let mut cookie = Cookie::new("refresh_token", token_claims.refresh_token);
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(true);
        let refresh_token_expires = _state
            .helpers
            .date_time
            .timestamp_to_offset_date_time(token_claims.refresh_token_expiration);
        match refresh_token_expires {
            Ok(offset) => cookie.set_expires(Expiration::DateTime(offset)),
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid refresh token expiration".to_string(),
                    ..Default::default()
                })
            }
        }

        let access_token_expires = match _state
            .helpers
            .date_time
            .timestamp_to_utc_date_time(token_claims.expiration)
        {
            Ok(offset) => offset,
            Err(_) => {
                return Err(HandlerError {
                    status: StatusCode::BAD_REQUEST,
                    message: "Invalid access token expiration".to_string(),
                    ..Default::default()
                })
            }
        };
        Ok((
            jar.add(cookie),
            Json(LoginResult {
                access_token: token_claims.access_token,
                expiration: access_token_expires,
            }),
        ))
    }
}

pub struct AuthenticationHandler {}
