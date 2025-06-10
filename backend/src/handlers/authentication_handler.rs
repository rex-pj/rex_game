use crate::{app_state::AppStateTrait, view_models::users::login_result::LoginResult};
use axum::{extract::State, http::HeaderMap, response::Result, Json};
use axum_extra::extract::{
    cookie::{Cookie, Expiration, SameSite},
    CookieJar,
};
use hyper::StatusCode;
use rex_game_application::{
    identities::identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    users::user_login_parameter::UserLoginParameter,
};
use rex_game_infrastructure::helpers::datetime_helper_trait::DateTimeHelperTrait;

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
        cookie.set_secure(true);
        let refresh_token_expires = _state
            .date_time_helper()
            .timestamp_to_offset_date_time(token_claims.refresh_token_expiration);
        match refresh_token_expires {
            Ok(offset) => cookie.set_expires(Expiration::DateTime(offset)),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }

        let access_token_expires = match _state
            .date_time_helper()
            .timestamp_to_utc_date_time(token_claims.expiration)
        {
            Ok(offset) => offset,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        };

        Ok((
            jar.add(cookie),
            Json(LoginResult {
                access_token: token_claims.access_token,
                expiration: access_token_expires,
            }),
        ))
    }

    pub async fn logout<T: AppStateTrait>(State(_state): State<T>, jar: CookieJar) ->  Result<(CookieJar, Json<bool>), StatusCode> {
        let mut cookie = Cookie::new("refresh_token", "");
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Lax);
        cookie.set_secure(true);
        cookie.set_max_age(time::Duration::ZERO);
        Ok((jar.remove(cookie), Json(true)))
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
        cookie.set_secure(true);
        let refresh_token_expires = _state
            .date_time_helper()
            .timestamp_to_offset_date_time(token_claims.refresh_token_expiration);
        match refresh_token_expires {
            Ok(offset) => cookie.set_expires(Expiration::DateTime(offset)),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }

        let access_token_expires = match _state
            .date_time_helper()
            .timestamp_to_utc_date_time(token_claims.expiration)
        {
            Ok(offset) => offset,
            Err(_) => return Err(StatusCode::BAD_REQUEST),
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
