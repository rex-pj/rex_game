use crate::{
    app_state::AppState, middlewares::AuthorizedState,
    view_models::users::current_user::CurrentUser,
};
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_identity::application::usecases::auth::{
    IdentityAuthenticateUseCaseTrait, IdentityUserUseCaseTrait,
};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

/// Authentication middleware layer - simplified version
#[derive(Clone)]
pub struct AuthenticateLayer {
    pub app_state: Arc<AppState>,
}

impl<S> Layer<S> for AuthenticateLayer {
    type Service = AuthenticateMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticateMiddleware {
            inner,
            app_state: self.app_state.clone(),
        }
    }
}

/// Authentication middleware service
#[derive(Clone)]
pub struct AuthenticateMiddleware<S> {
    inner: S,
    app_state: Arc<AppState>,
}

impl<S> Service<Request> for AuthenticateMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let auth_token = match get_auth_token(&req) {
            Some(token) => token.to_string(),
            None => {
                return Box::pin(async { Ok(unauthorized_response()) });
            }
        };

        // Verify the access token and extract user claims
        let user_claims = match self
            .app_state
            .usecases
            .identity_authenticate
            .validate_token(&auth_token)
        {
            Ok(claims) => claims,
            Err(_) => {
                return Box::pin(async { Ok(unauthorized_response()) });
            }
        };

        let user_id = user_claims.sub;
        let app_state = self.app_state.clone();
        let mut inner = self.inner.clone();
        let email = user_claims.email;

        Box::pin(async move {
            // Fetch current user with roles and permissions
            let current_user = match app_state
                .usecases
                .identity_user
                .get_logged_in_user(auth_token.as_str())
                .await
            {
                Ok(user) => user,
                Err(_) => return Ok(unauthorized_response()),
            };

            let email = match email {
                Some(e) => e,
                None => return Ok(unauthorized_response()),
            };

            let role_names: Vec<String> = current_user
                .roles
                .into_iter()
                .map(|role| role.role_name)
                .collect();
            let user_permission_codes: Vec<String> = current_user
                .permissions
                .into_iter()
                .map(|perm| perm.permisson_code)
                .collect();

            let user = Arc::new(CurrentUser {
                id: user_id,
                email,
                name: current_user.name,
                display_name: current_user.display_name,
                roles: role_names,
                permissions: user_permission_codes,
            });

            req.extensions_mut().insert(AuthorizedState::IsInRole);
            req.extensions_mut().insert(user);

            inner.call(req).await
        })
    }
}

/// Extract token from Authorization header (Bearer scheme)
fn get_auth_token(req: &Request) -> Option<&str> {
    req.headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
}

/// Build a generic unauthorized response
fn unauthorized_response() -> Response {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::from("Unauthorized"))
        .unwrap()
}
