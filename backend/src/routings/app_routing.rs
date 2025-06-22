use std::{collections::HashSet, sync::Arc};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use rex_game_application::users::roles::ROLE_ADMIN;
use tower::ServiceBuilder;

use crate::{
    app_state::RegularAppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, role_handler::RoleHandler,
        setup_handler::SetupHandler, user_handler::UserHandler,
    },
    middlewares::authenticate_middleware::AuthenticateLayer,
};

pub struct AppRouting {
    pub app_state: Arc<RegularAppState>,
}

impl AppRouting {
    pub fn build_authenticated_routes(
        &self,
        router: Router<RegularAppState>,
    ) -> Router<RegularAppState> {
        router
            .route(
                "/flashcards",
                post(FlashcardHandler::create_flashcard::<RegularAppState>),
            )
            .route(
                "/flashcards/{id}",
                patch(FlashcardHandler::update_flashcard::<RegularAppState>),
            )
            .route(
                "/flashcards/{id}",
                delete(FlashcardHandler::delete_flashcard::<RegularAppState>),
            )
            .route(
                "/flashcard-types",
                post(FlashcardTypeHandler::create_flashcard_type::<RegularAppState>),
            )
            .route(
                "/flashcard-types/{id}",
                patch(FlashcardTypeHandler::update_flashcard_type::<RegularAppState>),
            )
            .route(
                "/flashcard-types/{id}",
                delete(FlashcardTypeHandler::delete_flashcard_type::<RegularAppState>),
            )
            .route(
                "/auth/refresh",
                post(AuthenticationHandler::refresh_access_token::<RegularAppState>),
            )
            .route(
                "/auth/logout",
                delete(AuthenticationHandler::logout::<RegularAppState>),
            )
            .route(
                "/users/me",
                get(UserHandler::get_current_user::<RegularAppState>),
            )
            .route("/users", get(UserHandler::get_users::<RegularAppState>))
            .route(
                "/users/{id}",
                patch(UserHandler::update_user::<RegularAppState>),
            )
            .route(
                "/users/{id}",
                delete(UserHandler::delete_user::<RegularAppState>),
            )
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
                roles: Some(HashSet::from([])),
            }))
    }

    pub fn build_public_routes(&self, router: Router<RegularAppState>) -> Router<RegularAppState> {
        router
            .route(
                "/flashcards",
                get(FlashcardHandler::get_flashcards::<RegularAppState>),
            )
            .route(
                "/flashcards/{id}",
                get(FlashcardHandler::get_flashcard_by_id::<RegularAppState>),
            )
            .route(
                "/flashcards/images/{id}",
                get(FlashcardHandler::get_flashcard_image::<RegularAppState>),
            )
            .route(
                "/flashcard-types",
                get(FlashcardTypeHandler::get_flashcard_types::<RegularAppState>),
            )
            .route(
                "/flashcard-types/{id}",
                get(FlashcardTypeHandler::get_flashcard_type_by_id::<RegularAppState>),
            )
            .route(
                "/auth/login",
                post(AuthenticationHandler::login::<RegularAppState>),
            )
            .route("/users", post(UserHandler::create_user::<RegularAppState>))
            .route(
                "/users/{id}",
                get(UserHandler::get_user_by_id::<RegularAppState>),
            )
            .route("/setup", post(SetupHandler::setup::<RegularAppState>))
    }

    pub fn build_admin_routes(&self, router: Router<RegularAppState>) -> Router<RegularAppState> {
        router
            .route("/roles", get(RoleHandler::get_roles::<RegularAppState>))
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
                roles: Some(HashSet::from([ROLE_ADMIN.to_string()])),
            }))
    }
}
