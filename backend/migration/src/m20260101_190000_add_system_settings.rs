use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(SystemSettings::Key)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SystemSettings::Value).text().not_null())
                    .col(
                        ColumnDef::new(SystemSettings::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SystemSettings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SystemSettings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
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

#[derive(DeriveIden)]
enum SystemSettings {
    Table,
    Key,
    Value,
    Description,
    CreatedAt,
    UpdatedAt,
}
