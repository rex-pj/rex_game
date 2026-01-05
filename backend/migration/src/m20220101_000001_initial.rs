use async_trait::async_trait;
use sea_orm_migration::{
    prelude::*,
    schema::{pk_auto, string},
    sea_orm::Iterable,
};

use crate::enums::{
    flashcard::Flashcard, flashcard_file::FlashcardFile, flashcard_type::FlashcardType,
    flashcard_type_relation::FlashcardTypeRelation, role::Role, user::User, user_role::UserRole,
    user_status::UserStatus,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::DisplayName).string())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::SecurityStamp).string().not_null())
                    .col(ColumnDef::new(User::CreatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-created-by")
                            .from(User::Table, User::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(User::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::UpdatedById).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-updated-by")
                            .from(User::Table, User::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(User::StatusId)
                            .enumeration(Alias::new("UserStatus"), UserStatus::iter())
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

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
                    .col(ColumnDef::new(Role::IsActived).boolean().not_null())
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
                    .col(ColumnDef::new(UserRole::CreatedById).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-role-created-by-id")
                            .from(UserRole::Table, UserRole::CreatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(
                        ColumnDef::new(UserRole::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserRole::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserRole::UpdatedById).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-role-updated-by-id")
                            .from(UserRole::Table, UserRole::UpdatedById)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(UserRole::IsActived).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(FlashcardType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FlashcardType::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FlashcardType::Name).string().not_null())
                    .col(ColumnDef::new(FlashcardType::Description).string())
                    .col(
                        ColumnDef::new(FlashcardType::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardType::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardType::IsActived)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardType::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-type-created-by-id")
                            .from_tbl(FlashcardType::Table)
                            .from_col(FlashcardType::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .col(
                        ColumnDef::new(FlashcardType::UpdatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-type-updated-by-id")
                            .from_tbl(FlashcardType::Table)
                            .from_col(FlashcardType::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FlashcardFile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FlashcardFile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FlashcardFile::Name).string())
                    .col(ColumnDef::new(FlashcardFile::FileName).string().not_null())
                    .col(
                        ColumnDef::new(FlashcardFile::ContentType)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(FlashcardFile::Data).binary().not_null())
                    .col(
                        ColumnDef::new(FlashcardFile::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardFile::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardFile::IsActived)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardFile::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-file-created-by-id")
                            .from_tbl(FlashcardFile::Table)
                            .from_col(FlashcardFile::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .col(
                        ColumnDef::new(FlashcardFile::UpdatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-file-updated-by-id")
                            .from_tbl(FlashcardFile::Table)
                            .from_col(FlashcardFile::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Flashcard::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Flashcard::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Flashcard::Name).string().not_null())
                    .col(ColumnDef::new(Flashcard::Description).string())
                    .col(ColumnDef::new(Flashcard::SubDescription).string())
                    .col(
                        ColumnDef::new(Flashcard::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Flashcard::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Flashcard::FileId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-flashcard-flashcard_file")
                            .from(Flashcard::Table, Flashcard::FileId)
                            .to(FlashcardFile::Table, FlashcardFile::Id),
                    )
                    .col(ColumnDef::new(Flashcard::IsActived).boolean().not_null())
                    .col(ColumnDef::new(Flashcard::CreatedById).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-created-by-id")
                            .from_tbl(Flashcard::Table)
                            .from_col(Flashcard::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .col(ColumnDef::new(Flashcard::UpdatedById).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-updated-by-id")
                            .from_tbl(Flashcard::Table)
                            .from_col(Flashcard::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(FlashcardTypeRelation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::FlashcardId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::FlashcardTypeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-flashcard_type_relation-flashcard")
                            .from(
                                FlashcardTypeRelation::Table,
                                FlashcardTypeRelation::FlashcardId,
                            )
                            .to(Flashcard::Table, Flashcard::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-flashcard_type_relation-flashcard_type")
                            .from(
                                FlashcardTypeRelation::Table,
                                FlashcardTypeRelation::FlashcardTypeId,
                            )
                            .to(FlashcardType::Table, FlashcardType::Id),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::CreatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::UpdatedDate)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::CreatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-type-relation-created-by-id")
                            .from_tbl(FlashcardTypeRelation::Table)
                            .from_col(FlashcardTypeRelation::CreatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .col(
                        ColumnDef::new(FlashcardTypeRelation::UpdatedById)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-flashcard-type-relation-updated-by-id")
                            .from_tbl(FlashcardTypeRelation::Table)
                            .from_col(FlashcardTypeRelation::UpdatedById)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order to avoid foreign key constraint violations

        manager
            .drop_table(Table::drop().table(FlashcardTypeRelation::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Flashcard::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FlashcardFile::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FlashcardType::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}
