use crate::{
    app_state::AppStateTrait, middlewares::AuthorizedState,
    view_models::users::current_user::CurrentUser,
};
use axum::{body::Body, extract::Request, response::Response};
use hyper::StatusCode;
use rex_game_application::identities::{
    identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
    identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait,
    identity_user_usecase_trait::IdentityUserUseCaseTrait,
};
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
    pub roles: Option<Vec<String>>,
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
    _roles: Option<Vec<String>>,
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
        let auth_token = Self::get_auth_token(&req).unwrap_or("");

        // Verify the access token and extract user claims
        // If verification fails, return Unauthorized response
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

        let uer_id = user_claims.sub;
        let app_state = self._app_state.clone();
        let mut inner = self.inner.clone();
        let required_roles_option = self._roles.clone();
        let access_token = auth_token.to_string();

        Box::pin(async move {
            // If there are required roles, check if the user has any of them
            if let Some(roles) = required_roles_option {
                let is_authorized = app_state
                    .identity_authorize_usecase()
                    .is_user_in_role(uer_id, roles.into_iter().collect())
                    .await
                    .is_ok_and(|is_ok| is_ok);
                if !is_authorized {
                    return Self::unauthorized_response();
                }
            }

            let current_user = match app_state
                .identity_user_usecase()
                .get_logged_in_user(access_token.as_str())
                .await
            {
                Ok(user) => user,
                Err(_) => return Self::unauthorized_response(),
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
                id: uer_id,
                email: user_claims.email,
                roles: role_names,
                permissions: user_permission_codes,
            });
            req.extensions_mut().insert(AuthorizedState::IsInRole);
            req.extensions_mut().insert(user);
            inner.call(req).await
        })
    }
}

impl<S, T> AuthenticateMiddleware<S, T>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
{
    pub fn unauthorized_response() -> Result<S::Response, S::Error> {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Unauthorized"))
            .unwrap());
    }

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
