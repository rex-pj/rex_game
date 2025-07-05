pub use sea_orm_migration::prelude::*;

pub mod enums;
mod m20220101_000001_initial;
mod m20250309_074937_seeding_default_role;
mod m20250623_174418_add_permission;
mod m20250630_171738_seeding_default_permission;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial::Migration),
            Box::new(m20250309_074937_seeding_default_role::Migration),
            Box::new(m20250623_174418_add_permission::Migration),
            Box::new(m20250630_171738_seeding_default_permission::Migration),
        ]
    }
}
