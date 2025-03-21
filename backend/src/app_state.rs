use std::sync::Arc;

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
        identity_user_usecase::IdentityUserUseCase,
        identity_user_usecase_trait::IdentityUserUseCaseTrait,
    },
    users::{user_usecase::UserUseCase, user_usecase_trait::UserUseCaseTrait},
};
use rex_game_domain::helpers::file_helper_trait::FileHelperTrait;
use rex_game_infrastructure::{
    helpers::{
        configuration_helper::ConfigurationHelper, file_helper::FileHelper,
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
};
use sea_orm::DatabaseConnection;

pub trait AppStateTrait: Clone + Send + Sync + 'static {
    type FlashcardUseCase: FlashcardUseCaseTrait;
    type FlashcardTypeUseCase: FlashcardTypeUseCaseTrait;
    type UserUseCase: UserUseCaseTrait;
    type IdentityUserUseCase: IdentityUserUseCaseTrait;
    type IdentityAuthenticateUseCase: IdentityAuthenticateUseCaseTrait;
    type FileHelper: FileHelperTrait + FileHelperObjectTrait;
    fn flashcard_usecase(&self) -> &Self::FlashcardUseCase;
    fn flashcard_type_usecase(&self) -> &Self::FlashcardTypeUseCase;
    fn user_usecase(&self) -> &Self::UserUseCase;
    fn identity_user_usecase(&self) -> &Self::IdentityUserUseCase;
    fn identity_authenticate_usecase(&self) -> &Self::IdentityAuthenticateUseCase;
    fn file_helper(&self) -> &Self::FileHelper;
    fn db_connection(&self) -> &Arc<DatabaseConnection>;
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
    >,
    pub identity_authenticate_usecase: IdentityAuthenticateUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >,
    pub file_helper: FileHelper,
    pub db_connection: Arc<DatabaseConnection>,
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
    >;
    type IdentityAuthenticateUseCase = IdentityAuthenticateUseCase<
        IdentityPasswordHasher,
        UserUseCase<UserRepository, RoleRepository, UserRoleRepository>,
        IdentityTokenHelper<ConfigurationHelper>,
    >;
    type FileHelper = FileHelper;

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

    fn file_helper(&self) -> &Self::FileHelper {
        &self.file_helper
    }

    fn db_connection(&self) -> &Arc<DatabaseConnection> {
        &self.db_connection
    }
}
