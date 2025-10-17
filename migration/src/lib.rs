pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251016_170506_rsslite_base::Migration),
            Box::new(m20251017_005743_logs::Migration),
            Box::new(m20251017_013724_channel_add_ts::Migration),
        ]
    }
}
mod m20251016_170506_rsslite_base;
mod m20251017_005743_logs;
mod m20251017_013724_channel_add_ts;
