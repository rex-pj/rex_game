// Auth-related usecases
pub mod auth;

// Permission usecases
pub mod permission_creation_dto;
pub mod permission_deletion_dto;
pub mod permission_dto;
pub mod permission_updation_dto;
pub mod permission_usecase;
pub mod permission_usecase_trait;

// Role usecases
pub mod roles;
pub mod role_creation_dto;
pub mod role_deletion_dto;
pub mod role_dto;
pub mod role_permission_creation_dto;
pub mod role_permission_dto;
pub mod role_updation_dto;
pub mod role_usecase;
pub mod role_usecase_trait;

// User usecases
pub mod loggedin_user_dto;
pub mod user_creation_dto;
pub mod user_deletion_dto;
pub mod user_details_dto;
pub mod user_dto;
pub mod user_permission_creation_dto;
pub mod user_permission_dto;
pub mod user_role_creation_dto;
pub mod user_role_dto;
pub mod user_updation_dto;
pub mod user_usecase;
pub mod user_usecase_trait;

// Re-exports
pub use auth::*;
pub use permission_usecase::PermissionUseCase;
pub use permission_usecase_trait::PermissionUseCaseTrait;
pub use role_usecase::RoleUseCase;
pub use role_usecase_trait::RoleUseCaseTrait;
pub use user_usecase::UserUseCase;
pub use user_usecase_trait::UserUseCaseTrait;
