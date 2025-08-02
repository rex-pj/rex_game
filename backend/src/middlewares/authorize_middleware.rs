use crate::{app_state::AppStateTrait, middlewares::AuthorizedState};
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
        identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait,
    },
    users::{roles::ROLE_ADMIN, user_usecase_trait::UserUseCaseTrait},
};
use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{self, Layer, Service};

#[derive(Clone)]
pub struct AuthorizeLayer<T>
where
    T: AppStateTrait,
{
    pub app_state: Arc<T>,
    pub permissions: Option<Vec<String>>,
}

impl<S, T> Layer<S> for AuthorizeLayer<T>
where
    T: AppStateTrait,
{
    type Service = AuthorizeMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizeMiddleware {
            inner,
            _app_state: self.app_state.clone(),
            _permission_codes: self.permissions.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthorizeMiddleware<S, T> {
    pub inner: S,
    _app_state: Arc<T>,
    _permission_codes: Option<Vec<String>>,
}

impl<S, T> Service<Request> for AuthorizeMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    T: AppStateTrait + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let auth_token = Self::get_auth_token(&req).unwrap_or("");

        let user_claims = match self
            ._app_state
            .identity_authenticate_usecase()
            .verify_access_token(auth_token)
        {
            Ok(claims) => claims,
            Err(_) => {
                return Box::pin(async { Self::unauthorized_response() });
            }
        };

        let user_id = user_claims.sub;
        let app_state = self._app_state.clone();
        let mut inner = self.inner.clone();
        let permission_codes_option = self._permission_codes.clone();
        Box::pin(async move {
            if let Some(required_permission_codes) = permission_codes_option {
                let required_permissions: HashSet<String> =
                    required_permission_codes.into_iter().collect();

                let user_roles = app_state
                    .user_usecase()
                    .get_user_roles_by_user_id(user_id)
                    .await
                    .unwrap_or_default();

                if user_roles.iter().any(|role| role.role_name == ROLE_ADMIN) {
                    // If the user is a root admin, they have all permissions
                    req.extensions_mut().insert(AuthorizedState::HasPermission);
                    return inner.call(req).await;
                }

                // Check if the user has the required permissions
                let mut is_authorized = app_state
                    .identity_authorize_usecase()
                    .is_user_in_permission(user_id, required_permissions.to_owned())
                    .await
                    .is_ok_and(|is_ok| is_ok);

                // If the user does not have the required permissions, check their roles
                // to see if they have any roles that grant the required permissions
                if !is_authorized {
                    let user_role_ids: Vec<i32> = user_roles.iter().map(|f| f.id).collect();

                    // Check if the user's roles have the required permissions
                    is_authorized = app_state
                        .identity_authorize_usecase()
                        .are_roles_in_permission(user_role_ids, required_permissions)
                        .await
                        .is_ok_and(|is_ok| is_ok);
                }

                // If the user does not have the required permissions or roles, return Unauthorized
                if !is_authorized {
                    return Self::unauthorized_response();
                }
            }

            req.extensions_mut().insert(AuthorizedState::HasPermission);
            inner.call(req).await
        })
    }
}

impl<S, T> AuthorizeMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
{
    /// Returns an unauthorized response.
    pub fn unauthorized_response() -> Result<S::Response, S::Error> {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap());
    }

    /// Extracts the authentication token from the request headers.
    pub fn get_auth_token(req: &Request) -> Option<&str> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        // Extract the token from the Authorization header
        // The header should be in the format "Bearer <token>"
        let token_option = match auth_header {
            Some(header) => header.strip_prefix("Bearer "),
            None => None,
        };

        return token_option;
    }
}
