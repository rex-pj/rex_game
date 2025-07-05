use crate::{
    app_state::AppStateTrait, middlewares::AuthorizedState,
    view_models::users::current_user::CurrentUser,
};
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_application::{
    identities::{
        identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
        identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait,
    },
    users::user_usecase_trait::UserUseCaseTrait,
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
pub struct PermissionLayer<T>
where
    T: AppStateTrait,
{
    pub app_state: Arc<T>,
    pub permissions: Option<HashSet<String>>,
}

impl<S, T> Layer<S> for PermissionLayer<T>
where
    T: AppStateTrait,
{
    type Service = PermissionMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        PermissionMiddleware {
            inner,
            _app_state: self.app_state.clone(),
            _permission_codes: self.permissions.clone(),
        }
    }
}

#[derive(Clone)]
pub struct PermissionMiddleware<S, T> {
    pub inner: S,
    _app_state: Arc<T>,
    _permission_codes: Option<HashSet<String>>,
}

impl<S, T> Service<Request> for PermissionMiddleware<S, T>
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
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                header.strip_prefix("Bearer ").unwrap()
            }
            _ => {
                return Box::pin(async {
                    Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Unauthorized"))
                        .unwrap())
                })
            }
        };

        let current_user_claims = match self
            ._app_state
            .identity_authenticate_usecase()
            .verify_access_token(token)
        {
            Ok(claims) => CurrentUser {
                id: claims.sub,
                email: claims.email,
                roles: vec![],
                permissions: vec![],
            },
            Err(_) => {
                return Box::pin(async {
                    Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Unauthorized"))
                        .unwrap())
                });
            }
        };

        let app_state = self._app_state.clone();
        let mut inner = self.inner.clone();
        let permission_codes_option = self._permission_codes.clone();

        Box::pin(async move {
            if let Some(required_permission_codes) = permission_codes_option {
                let mut is_authorized = app_state
                    .identity_authorize_usecase()
                    .is_user_in_permission(
                        current_user_claims.id,
                        required_permission_codes.to_owned(),
                    )
                    .await
                    .is_ok();
                if !is_authorized {
                    let user_roles = app_state
                        .user_usecase()
                        .get_user_roles(current_user_claims.id)
                        .await;
                    let role_ids = match user_roles {
                        Ok(roles) => roles.into_iter().map(|f| f.role_id).collect(),
                        Err(_) => Vec::new(),
                    };
                    is_authorized = app_state
                        .identity_authorize_usecase()
                        .are_roles_in_permission(role_ids, required_permission_codes)
                        .await
                        .is_ok();
                }

                if !is_authorized {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Unauthorized"))
                        .unwrap());
                }
            }

            req.extensions_mut().insert(AuthorizedState::HasPermission);
            inner.call(req).await
        })
    }
}
