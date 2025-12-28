// New modular imports
use rex_game_games::{
    FlashcardFileRepository, FlashcardRepository, FlashcardTypeRelationRepository,
    FlashcardTypeRepository, FlashcardTypeUseCase, FlashcardUseCase,
};
use rex_game_identity::{
    IdentityAuthenticateUseCase, IdentityAuthorizeUseCase, IdentityPasswordHasher,
    IdentityTokenHelper, IdentityUserTokenUseCase, IdentityUserUseCase, PermissionRepository,
    PermissionUseCase, RolePermissionRepository, RoleRepository, RoleUseCase,
    UserPermissionRepository, UserRepository, UserRoleRepository, UserTokenRepository, UserUseCase,
};
use rex_game_mail_templates::MailTemplateUseCase;
use rex_game_shared_kernel::infrastructure::{
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        email_helper::EmailHelper, file_helper::FileHelper, html_helper::HtmlHelper,
    },
    transaction_manager::TransactionManager,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

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
        RoleUseCase<
            RoleRepository,
            PermissionRepository,
            RolePermissionRepository,
            UserRoleRepository,
        >,
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
    pub role: RoleUseCase<
        RoleRepository,
        PermissionRepository,
        RolePermissionRepository,
        UserRoleRepository,
    >,
    pub permission: PermissionUseCase<PermissionRepository>,
    pub identity_authorize: IdentityAuthorizeUseCase<
        UserRoleRepository,
        UserPermissionRepository,
        RolePermissionRepository,
    >,
    pub identity_user_token: IdentityUserTokenUseCase<UserTokenRepository>,
    pub mail_template: MailTemplateUseCase,
}

/// Group for all helper utilities
#[derive(Clone)]
pub struct Helpers {
    pub file: FileHelper,
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
}
