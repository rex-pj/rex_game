use crate::enums::{
    permission::Permission, role::Role, role_permission::RolePermission, user::User,
    user_permission::UserPermission,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(pk_auto(Permission::Id))
                    .col(string(Permission::Code))
                    .col(string(Permission::Name))
                    .col(string(Permission::Module))
                    .col(ColumnDef::new(Permission::Description).string().null())
                    .col(ColumnDef::new(Permission::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-permission-created-by-id")
                            .from(Permission::Table, Permission::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Permission::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Permission::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Permission::UpdatedById).integer())
                    .col(ColumnDef::new(Permission::IsActived).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-permission-updated-by-id")
                            .from(Permission::Table, Permission::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserPermission::Table)
                    .if_not_exists()
                    .col(pk_auto(UserPermission::Id))
                    .col(ColumnDef::new(UserPermission::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-permission-user-id")
                            .from(UserPermission::Table, UserPermission::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(UserPermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-permission-permission-id")
                            .from(UserPermission::Table, UserPermission::PermissionId)
                            .to(Permission::Table, Permission::Id),
                    )
                    .col(
                        ColumnDef::new(UserPermission::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-permission-created-by-id")
                            .from(UserPermission::Table, UserPermission::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(UserPermission::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPermission::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserPermission::UpdatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-permission-updated-by-id")
                            .from(UserPermission::Table, UserPermission::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(UserPermission::IsActived)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(pk_auto(RolePermission::Id))
                    .col(ColumnDef::new(RolePermission::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-permission-role-id")
                            .from(RolePermission::Table, RolePermission::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-permission-permission-id")
                            .from(RolePermission::Table, RolePermission::PermissionId)
                            .to(Permission::Table, Permission::Id),
                    )
                    .col(
                        ColumnDef::new(RolePermission::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-permission-created-by-id")
                            .from(RolePermission::Table, RolePermission::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(RolePermission::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::UpdatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-permission-updated-by-id")
                            .from(RolePermission::Table, RolePermission::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(RolePermission::IsActived)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to avoid foreign key constraint violations
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserPermission::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await?;

        Ok(())
    }
}
