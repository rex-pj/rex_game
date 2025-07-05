use chrono::Utc;
use sea_orm_migration::prelude::*;

use crate::enums::permission::Permission;

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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read permission".into(),
                "Can read the permission".into(),
                "permission".into(),
                "permission:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Creation permission".into(),
                "Can create the permission".into(),
                "permission".into(),
                "permission:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete permission".into(),
                "Can delete the permission".into(),
                "permission".into(),
                "permission:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update permission".into(),
                "Can update the permission".into(),
                "permission".into(),
                "permission:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user permission".into(),
                "Can read the user permission".into(),
                "user_permission".into(),
                "user_permission:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Creation user permission".into(),
                "Can create the user permission".into(),
                "user_permission".into(),
                "user_permission:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user permission".into(),
                "Can delete the user permission".into(),
                "user_permission".into(),
                "user_permission:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user permission".into(),
                "Update the user permission".into(),
                "user_permission".into(),
                "user_permission:update".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(user_permission_insert).await?;

        let role_insert = Query::insert()
            .into_table(Permission::Table)
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read role".into(),
                "Can read the role".into(),
                "role".into(),
                "role:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create role".into(),
                "Can create the role".into(),
                "role".into(),
                "role:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete role".into(),
                "Can delete the role".into(),
                "role".into(),
                "role:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update role".into(),
                "Can update the role".into(),
                "role".into(),
                "role:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user role".into(),
                "Can read the user role".into(),
                "user_role".into(),
                "user_role:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create user role".into(),
                "Can create the user role".into(),
                "user_role".into(),
                "user_role:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user role".into(),
                "Can delete the user role".into(),
                "user_role".into(),
                "user_role:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user role".into(),
                "Can update the user role".into(),
                "user_role".into(),
                "user_role:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read user".into(),
                "Can read the user".into(),
                "user".into(),
                "user:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create user".into(),
                "Can create the user".into(),
                "user".into(),
                "user:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete user".into(),
                "Can delete the user".into(),
                "user".into(),
                "user:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update user".into(),
                "Can update the user".into(),
                "user".into(),
                "user:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard type".into(),
                "Can read the flashcard type".into(),
                "flashcard_type".into(),
                "flashcard_type:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard type".into(),
                "Can create the flashcard type".into(),
                "flashcard_type".into(),
                "flashcard_type:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard type".into(),
                "Can delete the flashcard type".into(),
                "flashcard_type".into(),
                "flashcard_type:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard type".into(),
                "Can update the flashcard type".into(),
                "flashcard_type".into(),
                "flashcard_type:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard".into(),
                "Can read the flashcard".into(),
                "flashcard".into(),
                "flashcard:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard".into(),
                "Can create the flashcard".into(),
                "flashcard".into(),
                "flashcard:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard".into(),
                "Can delete the flashcard".into(),
                "flashcard".into(),
                "flashcard:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard".into(),
                "Can update the flashcard".into(),
                "flashcard".into(),
                "flashcard:update".into(),
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
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Read flashcard file".into(),
                "Can read the flashcard file".into(),
                "flashcard_file".into(),
                "flashcard_file:read".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Create flashcard file".into(),
                "Can create the flashcard file".into(),
                "flashcard_file".into(),
                "flashcard_file:create".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Delete flashcard file".into(),
                "Can delete the flashcard file".into(),
                "flashcard_file".into(),
                "flashcard_file:delete".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Permission::Name,
                Permission::Description,
                Permission::Module,
                Permission::Code,
                Permission::CreatedDate,
                Permission::UpdatedDate,
                Permission::IsActived,
            ])
            .values_panic([
                "Update flashcard file".into(),
                "Can update the flashcard file".into(),
                "flashcard_file".into(),
                "flashcard_file:update".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(flashcard_file_insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let deleted = Query::delete().from_table(Permission::Table).to_owned();

        manager.exec_stmt(deleted).await?;

        Ok(())
    }
}
