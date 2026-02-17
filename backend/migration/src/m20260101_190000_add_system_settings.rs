use crate::enums::{system_settings::SystemSettings, user::User};
use sea_orm_migration::{prelude::*, schema::pk_auto};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SystemSettings::Table)
                    .if_not_exists()
                    .col(pk_auto(SystemSettings::Id))
                    .col(
                        ColumnDef::new(SystemSettings::Key)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(SystemSettings::Value).text().not_null())
                    .col(ColumnDef::new(SystemSettings::Description).text().null())
                    .col(ColumnDef::new(SystemSettings::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-system-settings-created-by-id")
                            .from(SystemSettings::Table, SystemSettings::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(SystemSettings::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SystemSettings::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SystemSettings::UpdatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-system-settings-updated-by-id")
                            .from(SystemSettings::Table, SystemSettings::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(SystemSettings::IsActived)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // Insert is_installed flag
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(SystemSettings::Table)
                    .columns([
                        SystemSettings::Key,
                        SystemSettings::Value,
                        SystemSettings::Description,
                    ])
                    .values_panic([
                        "is_installed".into(),
                        "false".into(),
                        "Indicates whether the application has been installed and set up".into(),
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SystemSettings::Table).to_owned())
            .await?;

        Ok(())
    }
}
