use crate::app_state::AppStateTrait;
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_application::identities::identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait;
use std::{
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
        }
    }
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: i32,
    pub email: String,
}

#[derive(Clone)]
pub struct AuthenticateMiddleware<S, T> {
    pub inner: S,
    _app_state: Arc<T>,
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

        req.extensions_mut().insert(Arc::new(current_user));
        let future = self.inner.call(req);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}
