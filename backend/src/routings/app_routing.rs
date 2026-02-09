use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
    middleware,
};

use rex_game_shared::domain::enums::permission_codes::PermissionCodes;
use rex_game_identity::roles::ROLE_ROOT_ADMIN;
use tower::ServiceBuilder;

use crate::{
    app_state::AppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, mail_template_handler::MailTemplateHandler,
        permission_handler::PermissionHandler, role_handler::RoleHandler,
        scoring_handler::ScoringHandler, setup_handler::SetupHandler, user_handler::UserHandler,
    },
    middlewares::{
        authenticate_middleware::AuthenticateLayer,
        authorize_middleware::{AuthorizeByPermissionLayer, AuthorizeByRoleLayer},
    },
};

pub struct AppRouting {
    pub app_state: Arc<AppState>,
}

impl AppRouting {
    pub fn build_authenticated_routes(&self, router: Router<AppState>) -> Router<AppState> {
        router
            .route("/auth/logout", delete(AuthenticationHandler::logout))
            .route("/users/me", get(UserHandler::get_current_user))
            .route("/users", get(UserHandler::get_users))
            .route("/users/{id}", patch(UserHandler::update_user))
            // Scoring routes (authenticated)
            .route("/games/sessions", post(ScoringHandler::start_game_session))
            .route("/games/sessions/complete", post(ScoringHandler::complete_game_session))
            .route("/games/history", get(ScoringHandler::get_game_history))
            .route("/games/best", get(ScoringHandler::get_best_games))
            .route("/games/progress", get(ScoringHandler::get_game_progress))
            .route("/games/progress", post(ScoringHandler::save_game_progress))
            .route("/games/progress", delete(ScoringHandler::reset_game_progress))
            .route("/users/me/stats", get(ScoringHandler::get_my_stats))
            .route("/users/me/achievements", get(ScoringHandler::get_my_achievements))
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
            }))
    }

    pub fn build_public_routes(&self, router: Router<AppState>) -> Router<AppState> {
        let auth_limiter = self.app_state.rate_limiters.auth.clone();
        let api_limiter = self.app_state.rate_limiters.api.clone();
        let strict_limiter = self.app_state.rate_limiters.strict.clone();

        // Authentication routes with strict rate limiting (5 req/sec)
        let auth_routes = Router::new()
            .route("/auth/login", post(AuthenticationHandler::login))
            .route(
                "/auth/refresh",
                post(AuthenticationHandler::refresh_access_token),
            )
            .route("/users", post(UserHandler::create_user))
            .route("/users/confirmations", post(UserHandler::confirm_user))
            .route("/setup", post(SetupHandler::setup))
            .route_layer(middleware::from_fn(move |req, next| {
                let limiter = auth_limiter.clone();
                async move { limiter.middleware(req, next).await }
            }));

        // Password recovery with very strict rate limiting (3 req/min)
        let password_routes = Router::new()
            .route("/users/password", post(UserHandler::forgot_password))
            .route("/users/password", patch(UserHandler::reset_password))
            .route_layer(middleware::from_fn(move |req, next| {
                let limiter = strict_limiter.clone();
                async move { limiter.middleware(req, next).await }
            }));

        // General public routes with moderate rate limiting (30 req/sec)
        let general_routes = Router::new()
            .route("/flashcards", get(FlashcardHandler::get_flashcards))
            .route(
                "/flashcards/{id}",
                get(FlashcardHandler::get_flashcard_by_id),
            )
            .route(
                "/flashcards/images/{id}",
                get(FlashcardHandler::get_flashcard_image),
            )
            .route(
                "/flashcard-types",
                get(FlashcardTypeHandler::get_flashcard_types),
            )
            .route(
                "/flashcard-types/{id}",
                get(FlashcardTypeHandler::get_flashcard_type_by_id),
            )
            .route("/users/{id}", get(UserHandler::get_user_by_id))
            .route("/setup/status", get(SetupHandler::get_status))
            // Public scoring routes
            .route("/game-types", get(ScoringHandler::get_game_types))
            .route("/leaderboard", get(ScoringHandler::get_leaderboard))
            .route("/achievements", get(ScoringHandler::get_achievements))
            .route("/users/{user_id}/stats", get(ScoringHandler::get_user_stats))
            .route_layer(middleware::from_fn(move |req, next| {
                let limiter = api_limiter.clone();
                async move { limiter.middleware(req, next).await }
            }));

        router
            .merge(auth_routes)
            .merge(password_routes)
            .merge(general_routes)
    }

    pub fn build_admin_routes(&self, router: Router<AppState>) -> Router<AppState> {
        router
            .route(
                "/roles",
                get(RoleHandler::get_roles).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RoleRead.as_str().to_string()],
                }),
            )
            .route(
                "/roles/{id}",
                get(RoleHandler::get_role_by_id).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RoleRead.as_str().to_string()],
                }),
            )
            .route(
                "/roles/{id}",
                delete(RoleHandler::delete_role).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RoleDelete.as_str().to_string()],
                }),
            )
            .route(
                "/roles",
                post(RoleHandler::create_role).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RoleCreate.as_str().to_string()],
                }),
            )
            .route(
                "/roles/{id}",
                patch(RoleHandler::update_role).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RoleUpdate.as_str().to_string()],
                }),
            )
            .route(
                "/roles/{role_id}/permissions",
                post(RoleHandler::assign_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RolePermissionCreate.as_str().to_string()],
                }),
            )
            .route(
                "/roles/{role_id}/permissions",
                get(RoleHandler::get_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RolePermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/users/{id}",
                delete(UserHandler::delete_user).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserDelete.as_str().to_string()],
                }),
            )
            .route(
                "/users/{user_id}/roles",
                get(UserHandler::get_roles).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserRoleRead.as_str().to_string()],
                }),
            )
            .route(
                "/users/{user_id}/roles",
                post(UserHandler::assign_roles).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserRoleCreate.as_str().to_string()],
                }),
            )
            .route(
                "/permissions",
                get(PermissionHandler::get_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::PermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/permissions/{id}",
                get(PermissionHandler::get_permission_by_id).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::PermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/permissions/{id}",
                delete(PermissionHandler::delete_permission).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::PermissionDelete.as_str().to_string()],
                }),
            )
            .route(
                "/permissions",
                post(PermissionHandler::create_permission).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::PermissionCreate.as_str().to_string()],
                }),
            )
            .route(
                "/permissions/{id}",
                patch(PermissionHandler::update_permission).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::PermissionUpdate.as_str().to_string()],
                }),
            )
            .route(
                "/users/{user_id}/permissions",
                post(UserHandler::assign_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserPermissionCreate.as_str().to_string()],
                }),
            )
            .route(
                "/users/{user_id}/permissions",
                get(UserHandler::get_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserPermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/flashcards",
                post(FlashcardHandler::create_flashcard).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardCreate.as_str().to_string()],
                }),
            )
            .route(
                "/flashcards/{id}",
                patch(FlashcardHandler::update_flashcard).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardUpdate.as_str().to_string()],
                }),
            )
            .route(
                "/flashcards/{id}",
                delete(FlashcardHandler::delete_flashcard).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardDelete.as_str().to_string()],
                }),
            )
            .route(
                "/flashcard-types",
                post(FlashcardTypeHandler::create_flashcard_type).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::FlashcardTypeCreate
                            .as_str()
                            .to_string()],
                    },
                ),
            )
            .route(
                "/flashcard-types/{id}",
                patch(FlashcardTypeHandler::update_flashcard_type).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::FlashcardTypeUpdate
                            .as_str()
                            .to_string()],
                    },
                ),
            )
            .route(
                "/flashcard-types/{id}",
                delete(FlashcardTypeHandler::delete_flashcard_type).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::FlashcardTypeDelete
                            .as_str()
                            .to_string()],
                    },
                ),
            )
            .route(
                "/user-permissions",
                get(PermissionHandler::get_user_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserPermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/role-permissions",
                get(PermissionHandler::get_role_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RolePermissionRead.as_str().to_string()],
                }),
            )
            .route(
                "/user-roles",
                get(RoleHandler::get_user_roles).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserRoleRead.as_str().to_string()],
                }),
            )
            .route(
                "/mail-templates",
                get(MailTemplateHandler::get_mail_templates).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateRead.as_str().to_string()],
                }),
            )
            .route(
                "/mail-templates/{id}",
                get(MailTemplateHandler::get_mail_template_by_id).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::MailTemplateRead.as_str().to_string()],
                    },
                ),
            )
            .route(
                "/mail-templates/{id}",
                delete(MailTemplateHandler::delete_mail_template).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::MailTemplateDelete.as_str().to_string()],
                    },
                ),
            )
            .route(
                "/mail-templates",
                post(MailTemplateHandler::create_mail_template).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateCreate.as_str().to_string()],
                }),
            )
            .route(
                "/mail-templates/{id}",
                patch(MailTemplateHandler::update_mail_template).layer(
                    AuthorizeByPermissionLayer {
                        app_state: self.app_state.clone(),
                        permissions: vec![PermissionCodes::MailTemplateUpdate.as_str().to_string()],
                    },
                ),
            )
            .layer(ServiceBuilder::new().layer(AuthorizeByRoleLayer {
                app_state: self.app_state.clone(),
                roles: vec![ROLE_ROOT_ADMIN.to_string()],
            }))
    }
}
