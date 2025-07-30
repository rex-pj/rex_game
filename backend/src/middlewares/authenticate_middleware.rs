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
    roles::role_usecase_trait::RoleUseCaseTrait,
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
pub struct AuthenticateLayer<T>
where
    T: AppStateTrait,
{
    pub app_state: Arc<T>,
    pub roles: Option<HashSet<String>>,
}

impl<S, T> Layer<S> for AuthenticateLayer<T>
where
    T: AppStateTrait,
{
    type Service = AuthenticateMiddleware<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticateMiddleware {
            inner,
            _app_state: self.app_state.clone(),
            _roles: self.roles.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthenticateMiddleware<S, T> {
    pub inner: S,
    _app_state: Arc<T>,
    _roles: Option<HashSet<String>>,
}

impl<S, T> Service<Request> for AuthenticateMiddleware<S, T>
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
        let required_roles_option = self._roles.clone();

        Box::pin(async move {
            let user_permissions = app_state
                .user_usecase()
                .get_user_permissions_by_user_id(current_user_claims.id)
                .await;

            let mut user_permission_codes = match user_permissions {
                Ok(permissions) => permissions.into_iter().map(|f| f.permission_code).collect(),
                Err(_) => vec![],
            };

            let user_roles = match app_state
                .user_usecase()
                .get_user_roles_by_user_id(current_user_claims.id)
                .await
            {
                Ok(roles) => roles,
                Err(_) => vec![],
            };

            if !user_roles.is_empty() {
                let role_ids: Vec<i32> =
                    user_roles.clone().into_iter().map(|f| f.role_id).collect();
                if !role_ids.is_empty() {
                    let role_permissions = app_state
                        .role_usecase()
                        .get_roles_permissions_by_role_ids(role_ids)
                        .await;

                    if let Ok(permissions) = role_permissions {
                        permissions.into_iter().for_each(|f| {
                            user_permission_codes.push(f.permission_code);
                        });
                    }
                }
            }

            if let Some(required_roles) = required_roles_option {
                let authorized = app_state
                    .identity_authorize_usecase()
                    .is_user_in_role(current_user_claims.id, required_roles)
                    .await
                    .is_ok();
                if !authorized {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Unauthorized"))
                        .unwrap());
                }
            }

            let role_names = user_roles.into_iter().map(|f| f.role_name).collect();
            let user = Arc::new(CurrentUser {
                id: current_user_claims.id,
                email: current_user_claims.email,
                roles: role_names,
                permissions: user_permission_codes,
            });
            req.extensions_mut().insert(AuthorizedState::IsInRole);
            req.extensions_mut().insert(user);
            inner.call(req).await
        })
    }
}
