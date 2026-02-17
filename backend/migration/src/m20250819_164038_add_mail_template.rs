use crate::enums::{mail_template::MailTemplate, user::User};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MailTemplate::Table)
                    .if_not_exists()
                    .col(pk_auto(MailTemplate::Id))
                    .col(string(MailTemplate::Name))
                    .col(
                        ColumnDef::new(MailTemplate::Subject)
                            .string()
                            .not_null()
                            .string_len(255),
                    )
                    .col(ColumnDef::new(MailTemplate::Body).string().not_null())
                    .col(ColumnDef::new(MailTemplate::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-mail-template-created-by-id")
                            .from(MailTemplate::Table, MailTemplate::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(MailTemplate::CreatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MailTemplate::UpdatedOn)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MailTemplate::UpdatedById).integer())
                    .col(ColumnDef::new(MailTemplate::IsActived).boolean().not_null())
                    .col(ColumnDef::new(MailTemplate::IsEnabled).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-mail-template-updated-by-id")
                            .from(MailTemplate::Table, MailTemplate::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MailTemplate::Table).to_owned())
            .await
    }
}
