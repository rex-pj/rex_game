use crate::enums::permission::Permission;
use chrono::Utc;
use rex_game_shared::domain::enums::permission_codes::PermissionCodes;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now_utc = Utc::now().fixed_offset();

        let permission_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read permission".into(),
                "Can read the permission".into(),
                "permission".into(),
                PermissionCodes::PermissionRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Creation permission".into(),
                "Can create the permission".into(),
                "permission".into(),
                PermissionCodes::PermissionCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete permission".into(),
                "Can delete the permission".into(),
                "permission".into(),
                PermissionCodes::PermissionDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update permission".into(),
                "Can update the permission".into(),
                "permission".into(),
                PermissionCodes::PermissionUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(permission_insert).await?;

        let user_permission_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user permission".into(),
                "Can read the user permission".into(),
                "user_permission".into(),
                PermissionCodes::UserPermissionRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Creation user permission".into(),
                "Can create the user permission".into(),
                "user_permission".into(),
                PermissionCodes::UserPermissionCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user permission".into(),
                "Can delete the user permission".into(),
                "user_permission".into(),
                PermissionCodes::UserPermissionDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user permission".into(),
                "Update the user permission".into(),
                "user_permission".into(),
                PermissionCodes::UserPermissionUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(user_permission_insert).await?;

        let role_permission_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read role permission".into(),
                "Can read the role permission".into(),
                "role_permission".into(),
                PermissionCodes::RolePermissionRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Creation role permission".into(),
                "Can create the role permission".into(),
                "role_permission".into(),
                PermissionCodes::RolePermissionCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete role permission".into(),
                "Can delete the role permission".into(),
                "role_permission".into(),
                PermissionCodes::RolePermissionDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update role permission".into(),
                "Update the role permission".into(),
                "role_permission".into(),
                PermissionCodes::RolePermissionUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(role_permission_insert).await?;

        let role_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read role".into(),
                "Can read the role".into(),
                "role".into(),
                PermissionCodes::RoleRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create role".into(),
                "Can create the role".into(),
                "role".into(),
                PermissionCodes::RoleCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete role".into(),
                "Can delete the role".into(),
                "role".into(),
                PermissionCodes::RoleDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update role".into(),
                "Can update the role".into(),
                "role".into(),
                PermissionCodes::RoleUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(role_insert).await?;

        let user_role_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user role".into(),
                "Can read the user role".into(),
                "user_role".into(),
                PermissionCodes::UserRoleRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create user role".into(),
                "Can create the user role".into(),
                "user_role".into(),
                PermissionCodes::UserRoleCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user role".into(),
                "Can delete the user role".into(),
                "user_role".into(),
                PermissionCodes::UserRoleDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user role".into(),
                "Can update the user role".into(),
                "user_role".into(),
                PermissionCodes::UserRoleUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(user_role_insert).await?;

        let user_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user".into(),
                "Can read the user".into(),
                "user".into(),
                PermissionCodes::UserRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create user".into(),
                "Can create the user".into(),
                "user".into(),
                PermissionCodes::UserCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user".into(),
                "Can delete the user".into(),
                "user".into(),
                PermissionCodes::UserDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user".into(),
                "Can update the user".into(),
                "user".into(),
                PermissionCodes::UserUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(user_insert).await?;

        let flashcard_type_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard type".into(),
                "Can read the flashcard type".into(),
                "flashcard_type".into(),
                PermissionCodes::FlashcardTypeRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard type".into(),
                "Can create the flashcard type".into(),
                "flashcard_type".into(),
                PermissionCodes::FlashcardTypeCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard type".into(),
                "Can delete the flashcard type".into(),
                "flashcard_type".into(),
                PermissionCodes::FlashcardTypeDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard type".into(),
                "Can update the flashcard type".into(),
                "flashcard_type".into(),
                PermissionCodes::FlashcardTypeUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(flashcard_type_insert).await?;

        let flashcard_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard".into(),
                "Can read the flashcard".into(),
                "flashcard".into(),
                PermissionCodes::FlashcardRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard".into(),
                "Can create the flashcard".into(),
                "flashcard".into(),
                PermissionCodes::FlashcardCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard".into(),
                "Can delete the flashcard".into(),
                "flashcard".into(),
                PermissionCodes::FlashcardDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard".into(),
                "Can update the flashcard".into(),
                "flashcard".into(),
                PermissionCodes::FlashcardUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(flashcard_insert).await?;

        let flashcard_file_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard file".into(),
                "Can read the flashcard file".into(),
                "flashcard_file".into(),
                PermissionCodes::FlashcardFileRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard file".into(),
                "Can create the flashcard file".into(),
                "flashcard_file".into(),
                PermissionCodes::FlashcardFileCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard file".into(),
                "Can delete the flashcard file".into(),
                "flashcard_file".into(),
                PermissionCodes::FlashcardFileDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard file".into(),
                "Can update the flashcard file".into(),
                "flashcard_file".into(),
                PermissionCodes::FlashcardFileUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(flashcard_file_insert).await?;

        let mail_template_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Read mail template".into(),
                "Can read the mail template".into(),
                "Mail template".into(),
                PermissionCodes::MailTemplateRead.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Create mail template".into(),
                "Can create the mail template".into(),
                "Mail template".into(),
                PermissionCodes::MailTemplateCreate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete mail template".into(),
                "Can delete the mail template".into(),
                "Mail template".into(),
                PermissionCodes::MailTemplateDelete.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedOn,
                Permission::UpdatedOn,
                Permission::IsActived,
            ])
            .values_panic([
                "Update mail template".into(),
                "Can update the mail template".into(),
                "Mail template".into(),
                PermissionCodes::MailTemplateUpdate.as_str().into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(mail_template_insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let deleted = Query::delete().from_table(Permission::Table).to_owned();

        manager.exec_stmt(deleted).await?;

        Ok(())
    }
}
