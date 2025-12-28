// New modular imports
use rex_game_games::{
    FlashcardFileRepository, FlashcardRepository, FlashcardTypeRelationRepository,
    FlashcardTypeRepository, ScoringUseCase, {FlashcardTypeUseCase, FlashcardUseCase},
};
use rex_game_identity::{
    IdentityPasswordHasher, IdentityTokenHelper, PermissionRepository, RolePermissionRepository,
    RoleRepository, UserPermissionRepository, UserRepository, UserRoleRepository,
    UserTokenRepository,
    {
        IdentityAuthenticateUseCase, IdentityAuthorizeUseCase, IdentityUserTokenUseCase,
        IdentityUserUseCase, PermissionUseCase, RoleUseCase, UserUseCase,
    },
};
use rex_game_mail_templates::application::MailTemplateUseCase;
use rex_game_shared::infrastructure::{
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        email_helper::EmailHelper, html_helper::HtmlHelper,
    },
    transaction_manager::TransactionManager,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use crate::middlewares::rate_limit_middleware::RateLimiter;

/// Group for all use cases
#[derive(Clone)]
pub struct UseCases {
    pub flashcard: FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >,
    pub flashcard_type: FlashcardTypeUseCase<FlashcardTypeRepository>,
    pub user: UserUseCase<
        UserRepository,
        RoleRepository,
        UserRoleRepository,
        UserPermissionRepository,
        IdentityPasswordHasher,
    >,
    pub identity_user: IdentityUserUseCase<
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            UserPermissionRepository,
            IdentityPasswordHasher,
        >,
        RoleUseCase<RoleRepository, RolePermissionRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub identity_authenticate: IdentityAuthenticateUseCase<
        ConfigurationHelper,
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            UserPermissionRepository,
            IdentityPasswordHasher,
        >,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub role: RoleUseCase<RoleRepository, RolePermissionRepository, UserRoleRepository>,
    pub permission: PermissionUseCase<PermissionRepository>,
    pub identity_authorize: IdentityAuthorizeUseCase<
        UserRoleRepository,
        UserPermissionRepository,
        RolePermissionRepository,
    >,
    pub identity_user_token: IdentityUserTokenUseCase<UserTokenRepository>,
    pub mail_template: MailTemplateUseCase,
    pub scoring: ScoringUseCase,
}

/// Group for all helper utilities
#[derive(Clone)]
pub struct Helpers {
    pub email: EmailHelper,
    pub date_time: DateTimeHelper,
    pub html: HtmlHelper,
    pub configuration: Arc<ConfigurationHelper>,
    pub token: IdentityTokenHelper<ConfigurationHelper>,
}

/// Main application state combining use cases, helpers, and database connection
#[derive(Clone)]
pub struct AppState {
    pub usecases: UseCases,
    pub helpers: Helpers,
    pub db_connection: Arc<DatabaseConnection>,
    pub transaction_manager: TransactionManager,
    pub rate_limiters: RateLimiters,
}

/// Rate limiters for different endpoint categories
#[derive(Clone)]
pub struct RateLimiters {
    pub auth: RateLimiter,
    pub api: RateLimiter,
    pub strict: RateLimiter,
}
