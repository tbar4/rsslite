use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RssItem::Table)
                    .if_not_exists()
                    .col(pk_auto(RssItem::Id))
                    .col(string_null(RssItem::Title))
                    .col(string_uniq(RssItem::Link))
                    .col(string_null(RssItem::Description))
                    .col(string_null(RssItem::Author))
                    .col(string_null(RssItem::Categories))
                    .col(string_null(RssItem::Comments))
                    .col(string_null(RssItem::EnclosureUrl))
                    .col(string_null(RssItem::EnclosureLength))
                    .col(string_null(RssItem::EnclosureMimeType))
                    .col(string_null(RssItem::GUID))
                    .col(string_null(RssItem::PubDate))
                    .col(string_null(RssItem::SourceTitle))
                    .col(string_null(RssItem::SourceUrl))
                    .col(string_null(RssItem::Content))
                    //.col(string_null(RssItem::Extensions))
                    //.col(string_null(RssItem::ItunesExt))
                    //.col(string_null(RssItem::DublinCoreExt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RssItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RssItem {
    Table,
    Id,
    Title,
    Link,
    Description,
    Author,
    Categories,
    Comments,
    EnclosureUrl,
    EnclosureLength,
    EnclosureMimeType,
    GUID,
    PubDate,
    SourceTitle,
    SourceUrl,
    Content,
    //Extensions,
    //ItunesExt,
    //DublinCoreExt,
}
