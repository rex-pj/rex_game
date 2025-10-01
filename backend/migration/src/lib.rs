pub use sea_orm_migration::prelude::*;

pub mod enums;
mod m20220101_000001_initial;
mod m20250309_074937_seeding_default_role;
mod m20250623_174418_add_permission;
mod m20250630_171738_seeding_default_permission;
mod m20250817_075950_add_user_token;
mod m20250819_164038_add_mail_template;
mod m20250903_173631_seeding_email_templates;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial::Migration),
            Box::new(m20250309_074937_seeding_default_role::Migration),
            Box::new(m20250623_174418_add_permission::Migration),
            Box::new(m20250630_171738_seeding_default_permission::Migration),
            Box::new(m20250817_075950_add_user_token::Migration),
            Box::new(m20250819_164038_add_mail_template::Migration),
            Box::new(m20250903_173631_seeding_email_templates::Migration),
        ]
    }
}
