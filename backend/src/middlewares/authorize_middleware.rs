use crate::app_state::AppStateTrait;
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_application::identities::{
    identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait,
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
    pub roles: HashSet<String>,
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
            _roles: self.roles.clone(),
        }
    }
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
}

#[derive(Clone)]
pub struct AuthorizeMiddleware<S, T> {
    pub inner: S,
    _app_state: Arc<T>,
    _roles: HashSet<String>,
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

    fn call(&mut self, req: Request) -> Self::Future {
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
                        .status(401)
                        .body("Unauthorized".into())
                        .unwrap())
                })
            }
        };

        let current_user = match self
            ._app_state
            .identity_authenticate_usecase()
            .verify_access_token(token)
        {
            Ok(claims) => CurrentUser {
                id: claims.sub,
                email: claims.email,
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

        let role_names = self._roles.clone().into_iter().collect();
        let app_state = self._app_state.clone();
        let mut inner = self.inner.clone();

        return Box::pin(async move {
            let is_in_role = app_state
                .identity_authorize_usecase()
                .is_user_in_role(current_user.id, role_names);
            match is_in_role.await {
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from("Unauthorized"))
                        .unwrap());
                }
                Ok(_) => {
                    let response: Response = inner.call(req).await?;
                    Ok(response)
                }
            }
        });
    }
}
