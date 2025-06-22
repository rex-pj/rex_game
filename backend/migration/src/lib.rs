pub use sea_orm_migration::prelude::*;

pub mod enums;
mod m20220101_000001_initial;
mod m20250309_074937_seeding_default_role;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial::Migration),
            Box::new(m20250309_074937_seeding_default_role::Migration),
        ]
    }
}
