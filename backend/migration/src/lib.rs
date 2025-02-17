pub use sea_orm_migration::prelude::*;

mod m20220101_000001_initial;
mod m20250211_171706_add_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_initial::Migration),
            Box::new(m20250211_171706_add_user_table::Migration),
        ]
    }
}
