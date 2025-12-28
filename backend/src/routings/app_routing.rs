use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use rex_game_identity::roles::ROLE_ROOT_ADMIN;
use rex_game_shared_kernel::domain::enums::permission_codes::PermissionCodes;
use tower::ServiceBuilder;

use crate::{
    app_state::AppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, mail_template_handler::MailTemplateHandler,
        permission_handler::PermissionHandler, role_handler::RoleHandler,
        setup_handler::SetupHandler, user_handler::UserHandler,
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
            .route(
                "/auth/refresh",
                post(AuthenticationHandler::refresh_access_token),
            )
            .route("/auth/logout", delete(AuthenticationHandler::logout))
            .route("/users/me", get(UserHandler::get_current_user))
            .route("/users", get(UserHandler::get_users))
            .route("/users/{id}", patch(UserHandler::update_user))
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
            }))
    }

    pub fn build_public_routes(&self, router: Router<AppState>) -> Router<AppState> {
        router
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
            .route("/auth/login", post(AuthenticationHandler::login))
            .route("/users", post(UserHandler::create_user))
            .route("/users/password", post(UserHandler::forgot_password))
            .route("/users/password", patch(UserHandler::reset_password))
            .route("/users/confirmations", post(UserHandler::confirm_user))
            .route("/users/{id}", get(UserHandler::get_user_by_id))
            .route("/setup", post(SetupHandler::setup))
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
                    permissions: vec![PermissionCodes::RolePermissionCreate
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/roles/{role_id}/permissions",
                get(RoleHandler::get_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RolePermissionRead
                        .as_str()
                        .to_string()],
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
                    permissions: vec![PermissionCodes::UserPermissionCreate
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/users/{user_id}/permissions",
                get(UserHandler::get_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserPermissionRead
                        .as_str()
                        .to_string()],
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
                post(FlashcardTypeHandler::create_flashcard_type).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardTypeCreate
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/flashcard-types/{id}",
                patch(FlashcardTypeHandler::update_flashcard_type).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardTypeUpdate
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/flashcard-types/{id}",
                delete(FlashcardTypeHandler::delete_flashcard_type).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::FlashcardTypeDelete
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/user-permissions",
                get(PermissionHandler::get_user_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::UserPermissionRead
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/role-permissions",
                get(PermissionHandler::get_role_permissions).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::RolePermissionRead
                        .as_str()
                        .to_string()],
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
                get(MailTemplateHandler::get_mail_template_by_id).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateRead.as_str().to_string()],
                }),
            )
            .route(
                "/mail-templates/{id}",
                delete(MailTemplateHandler::delete_mail_template).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateDelete
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/mail-templates",
                post(MailTemplateHandler::create_mail_template).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateCreate
                        .as_str()
                        .to_string()],
                }),
            )
            .route(
                "/mail-templates/{id}",
                patch(MailTemplateHandler::update_mail_template).layer(AuthorizeByPermissionLayer {
                    app_state: self.app_state.clone(),
                    permissions: vec![PermissionCodes::MailTemplateUpdate
                        .as_str()
                        .to_string()],
                }),
            )
            .layer(ServiceBuilder::new().layer(AuthorizeByRoleLayer {
                app_state: self.app_state.clone(),
                roles: vec![ROLE_ROOT_ADMIN.to_string()],
            }))
    }
}
