use std::{collections::HashSet, sync::Arc};

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use rex_game_application::users::roles::ROLE_ROOT_ADMIN;
use tower::ServiceBuilder;

use crate::{
    app_state::RegularAppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, permission_handler::PermissionHandler,
        role_handler::RoleHandler, setup_handler::SetupHandler, user_handler::UserHandler,
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
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
                roles: None,
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
            .route(
                "/roles/{id}",
                get(RoleHandler::get_role_by_id::<RegularAppState>),
            )
            .route(
                "/roles/{id}",
                delete(RoleHandler::delete_role::<RegularAppState>),
            )
            .route("/roles", post(RoleHandler::create_role::<RegularAppState>))
            .route(
                "/roles/{id}",
                patch(RoleHandler::update_role::<RegularAppState>),
            )
            .route(
                "/roles/{role_id}/permissions",
                post(RoleHandler::assign_permissions::<RegularAppState>),
            )
            .route(
                "/roles/{role_id}/permissions",
                get(RoleHandler::get_permissions::<RegularAppState>),
            )
            .route(
                "/users/{id}",
                delete(UserHandler::delete_user::<RegularAppState>),
            )
            .route(
                "/users/{user_id}/roles",
                get(UserHandler::get_roles::<RegularAppState>),
            )
            .route(
                "/users/{user_id}/roles",
                post(UserHandler::assign_roles::<RegularAppState>),
            )
            .route(
                "/permissions",
                get(PermissionHandler::get_permissions::<RegularAppState>),
            )
            .route(
                "/permissions/{id}",
                get(PermissionHandler::get_permission_by_id::<RegularAppState>),
            )
            .route(
                "/permissions/{id}",
                delete(PermissionHandler::delete_permission::<RegularAppState>),
            )
            .route(
                "/permissions",
                post(PermissionHandler::create_permission::<RegularAppState>),
            )
            .route(
                "/permissions/{id}",
                patch(PermissionHandler::update_permission::<RegularAppState>),
            )
            .route(
                "/users/{user_id}/permissions",
                post(UserHandler::assign_permissions::<RegularAppState>),
            )
            .route(
                "/users/{user_id}/permissions",
                get(UserHandler::get_permissions::<RegularAppState>),
            )
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
                "/user-permissions",
                get(PermissionHandler::get_user_permissions::<RegularAppState>),
            )
            .route(
                "/role-permissions",
                get(PermissionHandler::get_role_permissions::<RegularAppState>),
            )
            .route(
                "/user-roles",
                get(RoleHandler::get_user_roles::<RegularAppState>),
            )
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
                roles: Some(HashSet::from([ROLE_ROOT_ADMIN.to_string()])),
            }))
    }
}
