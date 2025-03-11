use axum::{extract::State, http::HeaderMap, response::Result, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use hyper::StatusCode;
use rex_game_application::{
    identities::identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    users::user_login_parameter::UserLoginParameter,
};

use crate::{app_state::AppStateTrait, view_models::users::login_result::LoginResult};

impl AuthenticationHandler {
    pub async fn login<T: AppStateTrait>(
        State(_state): State<T>,
        jar: CookieJar,
        Json(payload): Json<Option<UserLoginParameter>>,
    ) -> Result<(CookieJar, Json<LoginResult>), StatusCode> {
        let req = match payload {
            Some(req) => req,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let token_claims = match _state
            .identity_authenticate_usecase()
            .password_login(&req.email, &req.password)
            .await
        {
            Ok(result) => result,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };

        let mut cookie = Cookie::new("refresh_token", token_claims.refresh_token);
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);

        Ok((
            jar.add(cookie),
            Json(LoginResult {
                access_token: token_claims.access_token,
            }),
        ))
    }

    pub async fn refresh_access_token<T: AppStateTrait>(
        headers: HeaderMap,
        State(_state): State<T>,
        jar: CookieJar,
    ) -> Result<(CookieJar, Json<LoginResult>), StatusCode> {
        let access_token_header = match headers.get("authorization") {
            Some(authorization) => authorization,
            None => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = match access_token_header.to_str() {
            Ok(authorization) => authorization,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };

        let access_token = access_token.strip_prefix("Bearer ").unwrap();
        let req_refresh_token = match jar.get("refresh_token") {
            Some(refresh_token) => refresh_token.value().to_string(),
            None => return Err(StatusCode::UNAUTHORIZED),
        };
        let token_claims = match _state
            .identity_authenticate_usecase()
            .refresh_access_token(&access_token, &req_refresh_token)
            .await
        {
            Ok(result) => result,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        };

        let mut cookie = Cookie::new("refresh_token", token_claims.refresh_token);
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);

        Ok((
            jar.add(cookie),
            Json(LoginResult {
                access_token: token_claims.access_token,
            }),
        ))
    }
}

pub struct AuthenticationHandler {}
