use std::sync::Arc;

use axum::{
    routing::{get, patch, post, put},
    Router,
};
use tower::ServiceBuilder;

use crate::{
    app_state::RegularAppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, setup_handler::SetupHandler,
        user_handler::UserHandler,
    },
    middlewares::auth_middleware::AuthLayer,
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
                "/flashcard-types",
                post(FlashcardTypeHandler::create_flashcard_type::<RegularAppState>),
            )
            .route(
                "/flashcard-types/{id}",
                put(FlashcardTypeHandler::update_flashcard_type::<RegularAppState>),
            )
            .route(
                "/auth/refresh",
                post(AuthenticationHandler::refresh_access_token::<RegularAppState>),
            )
            .layer(ServiceBuilder::new().layer(AuthLayer {
                app_state: self.app_state.clone(),
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
            .route("/setup", post(SetupHandler::setup::<RegularAppState>))
    }
}
