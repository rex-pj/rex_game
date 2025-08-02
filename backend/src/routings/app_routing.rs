use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use rex_game_application::users::roles::ROLE_ROOT_ADMIN;
use rex_game_shared::enums::permission_codes::PermissionCodes;
use tower::ServiceBuilder;

use crate::{
    app_state::RegularAppState,
    handlers::{
        authentication_handler::AuthenticationHandler, flashcard_handler::FlashcardHandler,
        flashcard_type_handler::FlashcardTypeHandler, permission_handler::PermissionHandler,
        role_handler::RoleHandler, setup_handler::SetupHandler, user_handler::UserHandler,
    },
    middlewares::{
        authenticate_middleware::AuthenticateLayer, authorize_middleware::AuthorizeLayer,
    },
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
            .route(
                "/roles",
                get(RoleHandler::get_roles::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RoleRead.as_str().to_string()]),
                }),
            )
            .route(
                "/roles/{id}",
                get(RoleHandler::get_role_by_id::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RoleRead.as_str().to_string()]),
                }),
            )
            .route(
                "/roles/{id}",
                delete(RoleHandler::delete_role::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RoleDelete.as_str().to_string()]),
                }),
            )
            .route(
                "/roles",
                post(RoleHandler::create_role::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RoleCreate.as_str().to_string()]),
                }),
            )
            .route(
                "/roles/{id}",
                patch(RoleHandler::update_role::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RoleUpdate.as_str().to_string()]),
                }),
            )
            .route(
                "/roles/{role_id}/permissions",
                post(RoleHandler::assign_permissions::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RolePermissionCreate
                        .as_str()
                        .to_string()]),
                }),
            )
            .route(
                "/roles/{role_id}/permissions",
                get(RoleHandler::get_permissions::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::RolePermissionRead
                        .as_str()
                        .to_string()]),
                }),
            )
            .route(
                "/users/{id}",
                delete(UserHandler::delete_user::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserDelete.as_str().to_string()]),
                }),
            )
            .route(
                "/users/{user_id}/roles",
                get(UserHandler::get_roles::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserRoleRead.as_str().to_string()]),
                }),
            )
            .route(
                "/users/{user_id}/roles",
                post(UserHandler::assign_roles::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserRoleCreate.as_str().to_string()]),
                }),
            )
            .route(
                "/permissions",
                get(PermissionHandler::get_permissions::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::PermissionRead.as_str().to_string()]),
                }),
            )
            .route(
                "/permissions/{id}",
                get(PermissionHandler::get_permission_by_id::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::PermissionRead
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/permissions/{id}",
                delete(PermissionHandler::delete_permission::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::PermissionDelete
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/permissions",
                post(PermissionHandler::create_permission::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::PermissionCreate
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/permissions/{id}",
                patch(PermissionHandler::update_permission::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::PermissionUpdate
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/users/{user_id}/permissions",
                post(UserHandler::assign_permissions::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserPermissionCreate
                        .as_str()
                        .to_string()]),
                }),
            )
            .route(
                "/users/{user_id}/permissions",
                get(UserHandler::get_permissions::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserPermissionRead
                        .as_str()
                        .to_string()]),
                }),
            )
            .route(
                "/flashcards",
                post(FlashcardHandler::create_flashcard::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::FlashcardCreate.as_str().to_string()]),
                }),
            )
            .route(
                "/flashcards/{id}",
                patch(FlashcardHandler::update_flashcard::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::FlashcardUpdate
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/flashcards/{id}",
                delete(FlashcardHandler::delete_flashcard::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::FlashcardDelete
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/flashcard-types",
                post(FlashcardTypeHandler::create_flashcard_type::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::FlashcardTypeCreate
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/flashcard-types/{id}",
                patch(FlashcardTypeHandler::update_flashcard_type::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::FlashcardTypeUpdate
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/flashcard-types/{id}",
                delete(FlashcardTypeHandler::delete_flashcard_type::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::FlashcardTypeDelete
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/user-permissions",
                get(PermissionHandler::get_user_permissions::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::UserPermissionRead
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/role-permissions",
                get(PermissionHandler::get_role_permissions::<RegularAppState>).layer(
                    AuthorizeLayer {
                        app_state: self.app_state.clone(),
                        permissions: Some(vec![PermissionCodes::RolePermissionRead
                            .as_str()
                            .to_string()]),
                    },
                ),
            )
            .route(
                "/user-roles",
                get(RoleHandler::get_user_roles::<RegularAppState>).layer(AuthorizeLayer {
                    app_state: self.app_state.clone(),
                    permissions: Some(vec![PermissionCodes::UserRoleRead.as_str().to_string()]),
                }),
            )
            .layer(ServiceBuilder::new().layer(AuthenticateLayer {
                app_state: self.app_state.clone(),
                roles: Some(vec![ROLE_ROOT_ADMIN.to_string()]),
            }))
    }
}
