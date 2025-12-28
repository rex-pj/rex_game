pub mod permission_repository_trait;
pub mod role_permission_repository_trait;
pub mod role_repository_trait;
pub mod user_permission_repository_trait;
pub mod user_repository_trait;
pub mod user_role_repository_trait;
pub mod user_token_repository_trait;

pub use permission_repository_trait::PermissionRepositoryTrait;
pub use role_permission_repository_trait::RolePermissionRepositoryTrait;
pub use role_repository_trait::RoleRepositoryTrait;
pub use user_permission_repository_trait::UserPermissionRepositoryTrait;
pub use user_repository_trait::UserRepositoryTrait;
pub use user_role_repository_trait::UserRoleRepositoryTrait;
pub use user_token_repository_trait::UserTokenRepositoryTrait;
