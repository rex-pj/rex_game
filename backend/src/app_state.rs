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
        identity_user_usecase::IdentityUserUseCase,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    roles::{role_usecase::RoleUseCase, role_usecase_trait::RoleUseCaseTrait},
    users::{user_usecase::UserUseCase, user_usecase_trait::UserUseCaseTrait},
};
use rex_game_domain::{
    helpers::file_helper_trait::FileHelperTrait, transaction_manager_trait::TransactionManagerTrait,
};
use rex_game_infrastructure::{
    helpers::{
        configuration_helper::ConfigurationHelper, datetime_helper::DateTimeHelper,
        datetime_helper_trait::DateTimeHelperTrait, file_helper::FileHelper,
        file_helper_object_trait::FileHelperObjectTrait,
    },
    identities::{
        identity_password_hasher::IdentityPasswordHasher,
        identity_token_helper::IdentityTokenHelper,
    },
    repositories::{
        flashcard_file_repository::FlashcardFileRepository,
        flashcard_repository::FlashcardRepository,
        flashcard_type_relation_repository::FlashcardTypeRelationRepository,
        flashcard_type_repository::FlashcardTypeRepository, role_repository::RoleRepository,
        user_repository::UserRepository, user_role_repository::UserRoleRepository,
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
    type FileHelper: FileHelperTrait + FileHelperObjectTrait;
    type DateTimeHelper: DateTimeHelperTrait;
    type TransactionManager: TransactionManagerTrait;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase;
    fn user_usecase(&self) -> &Self::UserUseCase;
    fn identity_user_usecase(&self) -> &Self::IdentityUserUseCase;
    fn identity_authenticate_usecase(&self) -> &Self::IdentityAuthenticateUseCase;
    fn identity_authorize_usecase(&self) -> &Self::IdentityAuthorizeUseCase;
    fn file_helper(&self) -> &Self::FileHelper;
    fn date_time_helper(&self) -> &Self::DateTimeHelper;
    fn db_connection(&self) -> &Arc<DatabaseConnection>;
    fn role_usecase(&self) -> &Self::RoleUseCase;
    fn transaction_manager(&self) -> &Self::TransactionManager;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub flashcard_usecase: FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >,
    pub flashcard_type_usecase: FlashcardTypeUseCase<FlashcardTypeRepository>,
    pub user_usecase: UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
    pub identity_user_usecase: IdentityUserUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub identity_authenticate_usecase: IdentityAuthenticateUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub file_helper: FileHelper,
    pub date_time_helper: DateTimeHelper,
    pub role_usecase: RoleUseCase<RoleRepository>,
    pub db_connection: Arc<DatabaseConnection>,
    pub identity_authorize_usecase: IdentityAuthorizeUseCase<UserRoleRepository>,
    pub transaction_manager: TransactionManager,
}

impl AppStateTrait for RegularAppState {
    type FlashcardUseCase = FlashcardUseCase<
        FlashcardRepository,
        FlashcardFileRepository,
        FlashcardTypeRelationRepository,
    >;
    type FlashcardTypeUseCase = FlashcardTypeUseCase<FlashcardTypeRepository>;
    type UserUseCase = UserUseCase<UserRepository, RoleRepository, UserRoleRepository>;
    type IdentityUserUseCase = IdentityUserUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >;
    type IdentityAuthenticateUseCase = IdentityAuthenticateUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >;
    type IdentityAuthorizeUseCase = IdentityAuthorizeUseCase<UserRoleRepository>;
    type FileHelper = FileHelper;
    type DateTimeHelper = DateTimeHelper;
    type RoleUseCase = RoleUseCase<RoleRepository>;
    type TransactionManager = TransactionManager;

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

    fn date_time_helper(&self) -> &Self::DateTimeHelper {
        &self.date_time_helper
    }

    fn db_connection(&self) -> &Arc<DatabaseConnection> {
        &self.db_connection
    }

    fn role_usecase(&self) -> &Self::RoleUseCase {
        &self.role_usecase
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.transaction_manager
    }
}
