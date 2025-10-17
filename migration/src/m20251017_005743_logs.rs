use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Log::Table)
                    .if_not_exists()
                    .col(pk_auto(Log::Id))
                    .col(string(Log::Ts))
                    .col(string(Log::Level))
                    .col(string(Log::Target))
                    .col(string(Log::Message))
                    .col(string_null(Log::Fields))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Log::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Log {
    Table,
    Id,
    Ts,
    Level,
    Target,
    Message,
    Fields,
}
