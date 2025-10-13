use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RssChannel::Table)
                    .if_not_exists()
                    // Primary key – automatically generated for SQLite
                    .col(pk_auto(RssChannel::Id))
                    // Simple scalar fields (always present)
                    .col(string(RssChannel::Title))
                    .col(string(RssChannel::Link).not_null().unique_key())
                    .col(string(RssChannel::Description))
                    // Optional scalar fields
                    .col(string_null(RssChannel::Language))
                    .col(string_null(RssChannel::Copyright))
                    .col(string_null(RssChannel::ManagingEditor))
                    .col(string_null(RssChannel::Webmaster))
                    .col(string_null(RssChannel::PubDate))
                    .col(string_null(RssChannel::LastBuildDate))
                    .col(string_null(RssChannel::Generator))
                    .col(string_null(RssChannel::Docs))
                    .col(string_null(RssChannel::Cloud)) // JSON
                    .col(string_null(RssChannel::Rating))
                    .col(string_null(RssChannel::Ttl))
                    .col(string_null(RssChannel::Image)) // JSON
                    .col(string_null(RssChannel::TextInput)) // JSON
                    // Collection / complex fields – stored as JSON strings
                    .col(string_null(RssChannel::Categories)) // JSON
                    .col(string_null(RssChannel::SkipHours)) // JSON (Vec<String>)
                    .col(string_null(RssChannel::SkipDays)) // JSON (Vec<String>)
                    .col(string_null(RssChannel::Items)) // JSON (Vec<rss::Item>)
                    .col(string_null(RssChannel::Extensions)) // JSON
                    .col(string_null(RssChannel::ItunesExt)) // JSON
                    .col(string_null(RssChannel::DublinCoreExt)) // JSON
                    .col(string_null(RssChannel::SyndicationExt)) // JSON
                    .col(string_null(RssChannel::Namespaces)) // JSON (BTreeMap)
                    // Timestamps – optional but handy
                    .col(
                        ColumnDef::new(RssChannel::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RssChannel::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RssChannel::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RssChannel {
    Table,
    Id,
    Title,
    Link,
    Description,
    Language,
    Copyright,
    ManagingEditor,
    Webmaster,
    PubDate,
    LastBuildDate,
    Categories,
    Generator,
    Docs,
    Cloud,
    Rating,
    Ttl,
    Image,
    TextInput,
    SkipHours,
    SkipDays,
    Items,
    Extensions,
    ItunesExt,
    DublinCoreExt,
    SyndicationExt,
    Namespaces,
    CreatedAt,
    UpdatedAt,
}
