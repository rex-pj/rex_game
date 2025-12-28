use crate::{app_state::AppState, middlewares::AuthorizedState, view_models::users::current_user::CurrentUser};
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_identity::application::usecases::{
    auth::{IdentityAuthenticateUseCaseTrait, IdentityAuthorizeUseCaseTrait},
    roles::ROLE_ROOT_ADMIN,
    UserUseCaseTrait,
};
use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

// ============================================================================
// AuthorizeByRoleLayer - Middleware for role-based authorization
// ============================================================================

/// Authorization middleware layer that checks if user has required roles
#[derive(Clone)]
pub struct AuthorizeByRoleLayer {
    pub app_state: Arc<AppState>,
    pub roles: Vec<String>,
}

impl<S> Layer<S> for AuthorizeByRoleLayer {
    type Service = AuthorizeByRoleMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizeByRoleMiddleware {
            inner,
            app_state: self.app_state.clone(),
            roles: self.roles.clone(),
        }
    }
}

/// Authorization middleware service for role checking
#[derive(Clone)]
pub struct AuthorizeByRoleMiddleware<S> {
    inner: S,
    app_state: Arc<AppState>,
    roles: Vec<String>,
}

impl<S> Service<Request> for AuthorizeByRoleMiddleware<S>
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
        let required_roles = self.roles.clone();

        Box::pin(async move {
            let is_authorized = app_state
                .usecases
                .identity_authorize
                .is_user_in_role(user_id, required_roles.into_iter().collect())
                .await
                .is_ok_and(|is_ok| is_ok);

            if !is_authorized {
                return Ok(forbidden_response());
            }

            req.extensions_mut()
                .insert(AuthorizedState::HasPermission);
            inner.call(req).await
        })
    }
}

// ============================================================================
// AuthorizeByPermissionLayer - Middleware for permission-based authorization
// ============================================================================

/// Authorization middleware layer that checks if user has required permissions
#[derive(Clone)]
pub struct AuthorizeByPermissionLayer {
    pub app_state: Arc<AppState>,
    pub permissions: Vec<String>,
}

impl<S> Layer<S> for AuthorizeByPermissionLayer {
    type Service = AuthorizeByPermissionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizeByPermissionMiddleware {
            inner,
            app_state: self.app_state.clone(),
            permissions: self.permissions.clone(),
        }
    }
}

/// Authorization middleware service for permission checking
#[derive(Clone)]
pub struct AuthorizeByPermissionMiddleware<S> {
    inner: S,
    app_state: Arc<AppState>,
    permissions: Vec<String>,
}

impl<S> Service<Request> for AuthorizeByPermissionMiddleware<S>
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
        let permission_codes = self.permissions.clone();

        Box::pin(async move {
            let required_permissions: HashSet<String> = permission_codes.into_iter().collect();

            // Get user info for CurrentUser extension
            let user_info = app_state
                .usecases
                .user
                .get_user_by_id(user_id)
                .await;

            let user = match user_info {
                Ok(u) => u,
                Err(_) => return Ok(unauthorized_response()),
            };

            let user_roles = app_state
                .usecases
                .user
                .get_user_roles_by_user_id(user_id)
                .await
                .unwrap_or_default();

            let role_names: Vec<String> = user_roles.iter().map(|r| r.role_name.clone()).collect();

            // Get user permissions
            let user_permissions = app_state
                .usecases
                .user
                .get_user_permissions_by_user_id(user_id)
                .await
                .unwrap_or_default();

            let permission_codes_list: Vec<String> = user_permissions.iter().map(|p| p.permission_code.clone()).collect();

            // Create CurrentUser
            let current_user = Arc::new(CurrentUser {
                id: user.id,
                email: user.email.clone(),
                name: user.name.clone(),
                display_name: user.display_name.clone(),
                roles: role_names.clone(),
                permissions: permission_codes_list,
            });

            // Root admin bypasses all permission checks
            if role_names.iter().any(|role| role == ROLE_ROOT_ADMIN) {
                req.extensions_mut()
                    .insert(AuthorizedState::HasPermission);
                req.extensions_mut().insert(current_user);
                return inner.call(req).await;
            }

            // Check user's direct permissions
            let has_permission = app_state
                .usecases
                .identity_authorize
                .is_user_in_permission(user_id, required_permissions.clone())
                .await
                .is_ok_and(|is_ok| is_ok);

            if has_permission {
                req.extensions_mut()
                    .insert(AuthorizedState::HasPermission);
                req.extensions_mut().insert(current_user);
                return inner.call(req).await;
            }

            // Check user's role-based permissions
            let user_role_ids: Vec<i32> = user_roles.iter().map(|f| f.id).collect();
            let has_role_permission = app_state
                .usecases
                .identity_authorize
                .are_roles_in_permission(user_role_ids, required_permissions)
                .await
                .is_ok_and(|is_ok| is_ok);

            if !has_role_permission {
                return Ok(forbidden_response());
            }

            req.extensions_mut()
                .insert(AuthorizedState::HasPermission);
            req.extensions_mut().insert(current_user);
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

/// Build a forbidden (insufficient permissions) response
fn forbidden_response() -> Response {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .body(Body::from("Forbidden"))
        .unwrap()
}
