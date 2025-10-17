use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251017_013724_channel_add_ts"
    }
}

#[derive(DeriveIden)]
enum RssChannel {
    Table,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Update existing records with current timestamp
        let db = manager.get_connection();
        db.execute_unprepared("UPDATE rss_channel SET created_at = datetime('now') WHERE created_at IS NULL")
            .await?;

        
        Ok(())
    }
}