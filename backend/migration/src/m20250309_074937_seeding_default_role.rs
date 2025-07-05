use crate::enums::role::Role;
use chrono::Utc;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now_utc = Utc::now().fixed_offset();

        let insert = Query::insert()
            .into_table(Role::Table)
            .columns([
                Role::Name,
                Role::Description,
                Role::CreatedDate,
                Role::UpdatedDate,
                Role::IsActived,
            ])
            .values_panic([
                "RootAdmin".into(),
                "The root administrator from the system".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Role::Name,
                Role::Description,
                Role::CreatedDate,
                Role::UpdatedDate,
                Role::IsActived,
            ])
            .values_panic([
                "Admin".into(),
                "The administrator".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Role::Name,
                Role::Description,
                Role::CreatedDate,
                Role::UpdatedDate,
                Role::IsActived,
            ])
            .values_panic([
                "ContentCreator".into(),
                "The content creator, who can create/update the content of the game".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .columns([
                Role::Name,
                Role::Description,
                Role::CreatedDate,
                Role::UpdatedDate,
                Role::IsActived,
            ])
            .values_panic([
                "Player".into(),
                "The player, who can only play the game".into(),
                now_utc.into(),
                now_utc.into(),
                true.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let deleted = Query::delete().from_table(Role::Table).to_owned();

        manager.exec_stmt(deleted).await?;

        Ok(())
    }
}
