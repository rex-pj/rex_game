use crate::enums::{role::Role, user::User, user_role::UserRole};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(pk_auto(Role::Id))
                    .col(string(Role::Name))
                    .col(ColumnDef::new(Role::Description).string().null())
                    .col(ColumnDef::new(Role::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-created-by-id")
                            .from(Role::Table, Role::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Role::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Role::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Role::UpdatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-updated-by-id")
                            .from(Role::Table, Role::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(pk_auto(UserRole::Id))
                    .col(ColumnDef::new(UserRole::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-role-user-id")
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(UserRole::RoleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-role-role-id")
                            .from(UserRole::Table, UserRole::RoleId)
                            .to(Role::Table, Role::Id),
                    )
                    .col(ColumnDef::new(Role::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-created-by-id")
                            .from(Role::Table, Role::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(Role::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Role::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Role::UpdatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-updated-by-id")
                            .from(Role::Table, Role::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        Ok(())
    }
}
