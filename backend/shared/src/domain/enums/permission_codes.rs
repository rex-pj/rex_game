#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionCodes {
    // Permission
    PermissionRead,
    PermissionCreate,
    PermissionDelete,
    PermissionUpdate,
    // User Permission
    UserPermissionRead,
    UserPermissionCreate,
    UserPermissionDelete,
    UserPermissionUpdate,
    // Role Permission
    RolePermissionRead,
    RolePermissionCreate,
    RolePermissionDelete,
    RolePermissionUpdate,
    // Role
    RoleRead,
    RoleCreate,
    RoleDelete,
    RoleUpdate,
    // User Role
    UserRoleRead,
    UserRoleCreate,
    UserRoleDelete,
    UserRoleUpdate,
    // User
    UserRead,
    UserCreate,
    UserDelete,
    UserUpdate,
    // Flashcard Type
    FlashcardTypeRead,
    FlashcardTypeCreate,
    FlashcardTypeDelete,
    FlashcardTypeUpdate,
    // Flashcard
    FlashcardRead,
    FlashcardCreate,
    FlashcardDelete,
    FlashcardUpdate,
    // Flashcard File
    FlashcardFileRead,
    FlashcardFileCreate,
    FlashcardFileDelete,
    FlashcardFileUpdate,
    // Mail Template
    MailTemplateRead,
    MailTemplateCreate,
    MailTemplateDelete,
    MailTemplateUpdate,
}

impl PermissionCodes {
    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionCodes::PermissionRead => "permission:read",
            PermissionCodes::PermissionCreate => "permission:create",
            PermissionCodes::PermissionDelete => "permission:delete",
            PermissionCodes::PermissionUpdate => "permission:update",

            PermissionCodes::UserPermissionRead => "user_permission:read",
            PermissionCodes::UserPermissionCreate => "user_permission:create",
            PermissionCodes::UserPermissionDelete => "user_permission:delete",
            PermissionCodes::UserPermissionUpdate => "user_permission:update",

            PermissionCodes::RolePermissionRead => "role_permission:read",
            PermissionCodes::RolePermissionCreate => "role_permission:create",
            PermissionCodes::RolePermissionDelete => "role_permission:delete",
            PermissionCodes::RolePermissionUpdate => "role_permission:update",

            PermissionCodes::RoleRead => "role:read",
            PermissionCodes::RoleCreate => "role:create",
            PermissionCodes::RoleDelete => "role:delete",
            PermissionCodes::RoleUpdate => "role:update",

            PermissionCodes::UserRoleRead => "user_role:read",
            PermissionCodes::UserRoleCreate => "user_role:create",
            PermissionCodes::UserRoleDelete => "user_role:delete",
            PermissionCodes::UserRoleUpdate => "user_role:update",

            PermissionCodes::UserRead => "user:read",
            PermissionCodes::UserCreate => "user:create",
            PermissionCodes::UserDelete => "user:delete",
            PermissionCodes::UserUpdate => "user:update",

            PermissionCodes::FlashcardTypeRead => "flashcard_type:read",
            PermissionCodes::FlashcardTypeCreate => "flashcard_type:create",
            PermissionCodes::FlashcardTypeDelete => "flashcard_type:delete",
            PermissionCodes::FlashcardTypeUpdate => "flashcard_type:update",

            PermissionCodes::FlashcardRead => "flashcard:read",
            PermissionCodes::FlashcardCreate => "flashcard:create",
            PermissionCodes::FlashcardDelete => "flashcard:delete",
            PermissionCodes::FlashcardUpdate => "flashcard:update",

            PermissionCodes::FlashcardFileRead => "flashcard_file:read",
            PermissionCodes::FlashcardFileCreate => "flashcard_file:create",
            PermissionCodes::FlashcardFileDelete => "flashcard_file:delete",
            PermissionCodes::FlashcardFileUpdate => "flashcard_file:update",

            PermissionCodes::MailTemplateRead => "mail_template:read",
            PermissionCodes::MailTemplateCreate => "mail_template:create",
            PermissionCodes::MailTemplateDelete => "mail_template:delete",
            PermissionCodes::MailTemplateUpdate => "mail_template:update",
        }
    }
}
