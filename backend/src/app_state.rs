use rex_game_application::{
    flashcard_types::{
        flashcard_type_usecase::FlashcardTypeUseCase,
        flashcard_type_usecase_trait::FlashcardTypeUseCaseTrait,
    },
    flashcards::{
        flashcard_usecase::FlashcardUseCase, flashcard_usecase_trait::FlashcardUseCaseTrait,
    },
    identities::{
        identity_authenticate_usecase::IdentityAuthenticateUseCase,
        identity_authenticate_usecase_trait::IdentityAuthenticateUseCaseTrait,
        identity_authorize_usecase::IdentityAuthorizeUseCase,
        identity_authorize_usecase_trait::IdentityAuthorizeUseCaseTrait,
        identity_user_token_usecase::IdentityUserTokenUseCase,
        identity_user_token_usecase_trait::IdentityUserTokenUseCaseTrait,
        identity_user_usecase::IdentityUserUseCase,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    mail_templates::{
        mail_template_usecase::MailTemplateUseCase,
        mail_template_usecase_trait::MailTemplateUseCaseTrait,
    },
    permissions::{
        permission_usecase::PermissionUseCase, permission_usecase_trait::PermissionUseCaseTrait,
    },
    roles::{role_usecase::RoleUseCase, role_usecase_trait::RoleUseCaseTrait},
    users::{user_usecase::UserUseCase, user_usecase_trait::UserUseCaseTrait},
};
use rex_game_domain::{
    helpers::{
        configuration_helper_trait::ConfigurationHelperTrait, email_helper_trait::EmailHelperTrait,
        file_helper_trait::FileHelperTrait,
    },
    identities::token_helper_trait::TokenHelperTrait,
    transaction_manager_trait::TransactionManagerTrait,
};
use rex_game_infrastructure::{
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        datetime_helper_trait::DateTimeHelperTrait, email_helper::EmailHelper,
        file_helper::FileHelper, file_helper_object_trait::FileHelperObjectTrait,
        html_helper::HtmlHelper, html_helper_trait::HtmlHelperTrait,
    },
    identities::{
        identity_password_hasher::IdentityPasswordHasher,
        identity_token_helper::IdentityTokenHelper,
    },
    repositories::{
        flashcard_file_repository::FlashcardFileRepository,
        flashcard_repository::FlashcardRepository,
        flashcard_type_relation_repository::FlashcardTypeRelationRepository,
        flashcard_type_repository::FlashcardTypeRepository,
        mail_template_repository::MailTemplateRepository,
        permission_repository::PermissionRepository,
        role_permission_repository::RolePermissionRepository, role_repository::RoleRepository,
        user_permission_repository::UserPermissionRepository, user_repository::UserRepository,
        user_role_repository::UserRoleRepository, user_token_repository::UserTokenRepository,
    },
    transaction_manager::TransactionManager,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub trait AppStateTrait: Clone + Send + Sync + 'static {
    type FlashcardUseCase: FlashcardUseCaseTrait;
    type FlashcardTypeUseCase: FlashcardTypeUseCaseTrait;
    type UserUseCase: UserUseCaseTrait;
    type IdentityUserUseCase: IdentityUserUseCaseTrait;
    type IdentityAuthenticateUseCase: IdentityAuthenticateUseCaseTrait;
    type IdentityAuthorizeUseCase: IdentityAuthorizeUseCaseTrait;
    type RoleUseCase: RoleUseCaseTrait;
    type PermissionUseCase: PermissionUseCaseTrait;
    type FileHelper: FileHelperTrait + FileHelperObjectTrait;
    type DateTimeHelper: DateTimeHelperTrait;
    type EmailHelper: EmailHelperTrait;
    type HtmlHelper: HtmlHelperTrait;
    type TransactionManager: TransactionManagerTrait;
    type IdentityUserTokenUseCase: IdentityUserTokenUseCaseTrait;
    type TokenHelper: TokenHelperTrait;
    type MailTemplateUseCase: MailTemplateUseCaseTrait;
    type ConfigurationHelper: ConfigurationHelperTrait;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase;
    fn user_usecase(&self) -> &Self::UserUseCase;
    fn identity_user_usecase(&self) -> &Self::IdentityUserUseCase;
    fn identity_authenticate_usecase(&self) -> &Self::IdentityAuthenticateUseCase;
    fn identity_authorize_usecase(&self) -> &Self::IdentityAuthorizeUseCase;
    fn file_helper(&self) -> &Self::FileHelper;
    fn email_helper(&self) -> &Self::EmailHelper;
    fn date_time_helper(&self) -> &Self::DateTimeHelper;
    fn html_helper(&self) -> &HtmlHelper;
    fn db_connection(&self) -> &Arc<DatabaseConnection>;
    fn role_usecase(&self) -> &Self::RoleUseCase;
    fn permission_usecase(&self) -> &Self::PermissionUseCase;
    fn transaction_manager(&self) -> &Self::TransactionManager;
    fn identity_user_token_usecase(&self) -> &Self::IdentityUserTokenUseCase;
    fn identity_token_helper(&self) -> &Self::TokenHelper;
    fn mail_template_usecase(&self) -> &Self::MailTemplateUseCase;
    fn configuration_helper(&self) -> &Self::ConfigurationHelper;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub flashcard_usecase: FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >,
    pub flashcard_type_usecase: FlashcardTypeUseCase<FlashcardTypeRepository>,
    pub user_usecase: UserUseCase<
        UserRepository,
        RoleRepository,
        UserRoleRepository,
        PermissionRepository,
        UserPermissionRepository,
        IdentityPasswordHasher,
    >,
    pub identity_user_usecase: IdentityUserUseCase<
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            PermissionRepository,
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
    pub identity_authenticate_usecase: IdentityAuthenticateUseCase<
        ConfigurationHelper,
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            PermissionRepository,
            UserPermissionRepository,
            IdentityPasswordHasher,
        >,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub file_helper: FileHelper,
    pub email_helper: EmailHelper,
    pub date_time_helper: DateTimeHelper,
    pub html_helper: HtmlHelper,
    pub role_usecase: RoleUseCase<
        RoleRepository,
        PermissionRepository,
        RolePermissionRepository,
        UserRoleRepository,
    >,
    pub permission_usecase: PermissionUseCase<PermissionRepository>,
    pub db_connection: Arc<DatabaseConnection>,
    pub identity_authorize_usecase: IdentityAuthorizeUseCase<
        UserRoleRepository,
        UserPermissionRepository,
        RolePermissionRepository,
    >,
    pub transaction_manager: TransactionManager,
    pub identity_user_token_usecase: IdentityUserTokenUseCase<UserTokenRepository>,
    pub itentity_token_helper: IdentityTokenHelper<ConfigurationHelper>,
    pub mail_template_usecase: MailTemplateUseCase<MailTemplateRepository>,
    pub configuration_helper: Arc<ConfigurationHelper>,
}

impl AppStateTrait for RegularAppState {
    type FlashcardUseCase = FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >;
    type FlashcardTypeUseCase = FlashcardTypeUseCase<FlashcardTypeRepository>;
    type UserUseCase = UserUseCase<
        UserRepository,
        RoleRepository,
        UserRoleRepository,
        PermissionRepository,
        UserPermissionRepository,
        IdentityPasswordHasher,
    >;
    type IdentityUserUseCase = IdentityUserUseCase<
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            PermissionRepository,
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
    >;
    type IdentityAuthenticateUseCase = IdentityAuthenticateUseCase<
        ConfigurationHelper,
        IdentityPasswordHasher,
        UserUseCase<
            UserRepository,
            RoleRepository,
            UserRoleRepository,
            PermissionRepository,
            UserPermissionRepository,
            IdentityPasswordHasher,
        >,
        IdentityTokenHelper<ConfigurationHelper>,
    >;
    type IdentityAuthorizeUseCase = IdentityAuthorizeUseCase<
        UserRoleRepository,
        UserPermissionRepository,
        RolePermissionRepository,
    >;
    type FileHelper = FileHelper;
    type EmailHelper = EmailHelper;
    type DateTimeHelper = DateTimeHelper;
    type HtmlHelper = HtmlHelper;
    type ConfigurationHelper = ConfigurationHelper;
    type RoleUseCase = RoleUseCase<
        RoleRepository,
        PermissionRepository,
        RolePermissionRepository,
        UserRoleRepository,
    >;
    type PermissionUseCase = PermissionUseCase<PermissionRepository>;
    type TransactionManager = TransactionManager;
    type IdentityUserTokenUseCase = IdentityUserTokenUseCase<UserTokenRepository>;
    type TokenHelper = IdentityTokenHelper<ConfigurationHelper>;
    type MailTemplateUseCase = MailTemplateUseCase<MailTemplateRepository>;

    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase {
        &self.flashcard_usecase
    }

    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase {
        &self.flashcard_type_usecase
    }

    fn user_usecase(&self) -> &Self::UserUseCase {
        &self.user_usecase
    }

    fn identity_user_usecase(&self) -> &Self::IdentityUserUseCase {
        &self.identity_user_usecase
    }

    fn identity_authenticate_usecase(&self) -> &Self::IdentityAuthenticateUseCase {
        &self.identity_authenticate_usecase
    }

    fn identity_authorize_usecase(&self) -> &Self::IdentityAuthorizeUseCase {
        &self.identity_authorize_usecase
    }

    fn file_helper(&self) -> &Self::FileHelper {
        &self.file_helper
    }

    fn email_helper(&self) -> &Self::EmailHelper {
        &self.email_helper
    }

    fn date_time_helper(&self) -> &Self::DateTimeHelper {
        &self.date_time_helper
    }

    fn html_helper(&self) -> &HtmlHelper {
        &self.html_helper
    }

    fn db_connection(&self) -> &Arc<DatabaseConnection> {
        &self.db_connection
    }

    fn role_usecase(&self) -> &Self::RoleUseCase {
        &self.role_usecase
    }

    fn permission_usecase(&self) -> &Self::PermissionUseCase {
        &self.permission_usecase
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.transaction_manager
    }

    fn identity_user_token_usecase(&self) -> &Self::IdentityUserTokenUseCase {
        &self.identity_user_token_usecase
    }

    fn identity_token_helper(&self) -> &Self::TokenHelper {
        &self.itentity_token_helper
    }

    fn mail_template_usecase(&self) -> &Self::MailTemplateUseCase {
        &self.mail_template_usecase
    }

    fn configuration_helper(&self) -> &Self::ConfigurationHelper {
        &self.configuration_helper
    }
}
