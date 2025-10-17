use sea_orm_migration::schema::json;
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
                    .col(pk_auto(RssChannel::Id))
                    .col(string(RssChannel::Title))
                    .col(string(RssChannel::Link))
                    .col(string(RssChannel::Description))
                    .col(string_null(RssChannel::Language))
                    .col(string_null(RssChannel::Copyright))
                    .col(string_null(RssChannel::ManagingEditor))
                    .col(string_null(RssChannel::Webmaster))
                    .col(string_null(RssChannel::PubDate))
                    .col(string_null(RssChannel::LastBuildDate))
                    .col(string_null(RssChannel::Generator))
                    .col(string_null(RssChannel::Docs))
                    .col(string_null(RssChannel::Rating))
                    .col(string_null(RssChannel::Ttl))
                    .col(timestamp(RssChannel::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssChannel::UpdatedAt).default(Expr::current_timestamp()))
                    .col(json(RssChannel::SkipHours))
                    .col(json(RssChannel::SkipDays))
                    .col(json(RssChannel::Extensions))
                    .col(json_null(RssChannel::Namespaces))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssItem::Table)
                    .if_not_exists()
                    .col(pk_auto(RssItem::Id))
                    .col(integer(RssItem::ChannelId))
                    .col(string_null(RssItem::Title))
                    .col(string_null(RssItem::Link))
                    .col(string_null(RssItem::Description))
                    .col(string_null(RssItem::Author))
                    .col(string_null(RssItem::Comments))
                    .col(string_null(RssItem::PubDate))
                    .col(string_null(RssItem::Content))
                    .col(timestamp(RssItem::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssItem::UpdatedAt).default(Expr::current_timestamp()))
                    .col(json(RssItem::Extensions))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-channel")
                            .from(RssItem::Table, RssItem::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssCategory::Table)
                    .if_not_exists()
                    .col(pk_auto(RssCategory::Id))
                    .col(integer_null(RssCategory::ChannelId))
                    .col(integer_null(RssCategory::ItemId))
                    .col(string(RssCategory::Name))
                    .col(string_null(RssCategory::Domain))
                    .col(timestamp(RssCategory::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssCategory::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-channel")
                            .from(RssCategory::Table, RssCategory::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-item")
                            .from(RssCategory::Table, RssCategory::ItemId)
                            .to(RssItem::Table, RssItem::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssItemEnclosure::Table)
                    .if_not_exists()
                    .col(pk_auto(RssItemEnclosure::Id))
                    .col(integer(RssItemEnclosure::ItemId))
                    .col(string(RssItemEnclosure::Url))
                    .col(string(RssItemEnclosure::Length))
                    .col(string(RssItemEnclosure::MimeType))
                    .col(timestamp(RssItemEnclosure::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssItemEnclosure::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-enclosure-item")
                            .from(RssItemEnclosure::Table, RssItemEnclosure::ItemId)
                            .to(RssItem::Table, RssItem::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssItemGuid::Table)
                    .if_not_exists()
                    .col(pk_auto(RssItemGuid::Id))
                    .col(integer(RssItemGuid::ItemId))
                    .col(string(RssItemGuid::Value))
                    .col(string(RssItemGuid::Permalink))
                    .col(timestamp(RssItemGuid::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssItemGuid::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-guid-item")
                            .from(RssItemGuid::Table, RssItemGuid::ItemId)
                            .to(RssItemGuid::Table, RssItemGuid::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssItunesItemExtension::Table)
                    .if_not_exists()
                    .col(pk_auto(RssItunesItemExtension::Id))
                    .col(integer(RssItunesItemExtension::ItemId))
                    .col(string_null(RssItunesItemExtension::Author))
                    .col(string_null(RssItunesItemExtension::Block))
                    .col(string_null(RssItunesItemExtension::Image))
                    .col(string_null(RssItunesItemExtension::Duration))
                    .col(string_null(RssItunesItemExtension::Explicit))
                    .col(string_null(RssItunesItemExtension::ClosedCaption))
                    .col(string_null(RssItunesItemExtension::Order))
                    .col(string_null(RssItunesItemExtension::Subtitle))
                    .col(string_null(RssItunesItemExtension::Summary))
                    .col(string_null(RssItunesItemExtension::Keywords))
                    .col(
                        timestamp(RssItunesItemExtension::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(RssItunesItemExtension::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(string_null(RssItunesItemExtension::Episode))
                    .col(string_null(RssItunesItemExtension::Season))
                    .col(string_null(RssItunesItemExtension::EpisodeType))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-itunes-item")
                            .from(
                                RssItunesItemExtension::Table,
                                RssItunesItemExtension::ItemId,
                            )
                            .to(RssItem::Table, RssItem::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssDublinCoreExt::Table)
                    .if_not_exists()
                    .col(pk_auto(RssDublinCoreExt::Id))
                    .col(integer(RssDublinCoreExt::ItemId))
                    .col(json(RssDublinCoreExt::Contributors))
                    .col(json(RssDublinCoreExt::Coverages))
                    .col(json(RssDublinCoreExt::Creators))
                    .col(json(RssDublinCoreExt::Dates))
                    .col(json(RssDublinCoreExt::Descriptions))
                    .col(json(RssDublinCoreExt::Formats))
                    .col(json(RssDublinCoreExt::Identifiers))
                    .col(json(RssDublinCoreExt::Languages))
                    .col(json(RssDublinCoreExt::Publishers))
                    .col(json(RssDublinCoreExt::Relations))
                    .col(json(RssDublinCoreExt::Rights))
                    .col(json(RssDublinCoreExt::Sources))
                    .col(json(RssDublinCoreExt::Subjects))
                    .col(json(RssDublinCoreExt::Titles))
                    .col(json(RssDublinCoreExt::Types))
                    .col(timestamp(RssDublinCoreExt::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssDublinCoreExt::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-dublincore-item")
                            .from(RssDublinCoreExt::Table, RssDublinCoreExt::ItemId)
                            .to(RssItem::Table, RssItem::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssCloud::Table)
                    .if_not_exists()
                    .col(pk_auto(RssCloud::Id))
                    .col(integer(RssCloud::ChannelId))
                    .col(string(RssCloud::Domain))
                    .col(string(RssCloud::Port))
                    .col(string(RssCloud::Path))
                    .col(string(RssCloud::RegisterProcedure))
                    .col(string(RssCloud::Protocol))
                    .col(timestamp(RssCloud::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssCloud::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cloud-channel")
                            .from(RssCloud::Table, RssCloud::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssImage::Table)
                    .if_not_exists()
                    .col(pk_auto(RssImage::Id))
                    .col(integer(RssImage::ChannelId))
                    .col(string(RssImage::Url))
                    .col(string(RssImage::Title))
                    .col(string(RssImage::Link))
                    .col(string_null(RssImage::Width))
                    .col(string_null(RssImage::Height))
                    .col(string(RssImage::Description))
                    .col(timestamp(RssImage::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssImage::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-image-channel")
                            .from(RssImage::Table, RssImage::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssTextInput::Table)
                    .if_not_exists()
                    .col(pk_auto(RssTextInput::Id))
                    .col(integer(RssTextInput::ChannelId))
                    .col(string(RssTextInput::Title))
                    .col(string(RssTextInput::Description))
                    .col(string(RssTextInput::Name))
                    .col(string(RssTextInput::Link))
                    .col(timestamp(RssTextInput::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssTextInput::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-text-channel")
                            .from(RssTextInput::Table, RssTextInput::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssSyndicationExt::Table)
                    .if_not_exists()
                    .col(pk_auto(RssSyndicationExt::Id))
                    .col(integer(RssSyndicationExt::ChannelId))
                    .col(string(RssSyndicationExt::Period))
                    .col(integer(RssSyndicationExt::Frequency))
                    .col(string(RssSyndicationExt::Base))
                    .col(timestamp(RssSyndicationExt::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(RssSyndicationExt::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-syndication-channel")
                            .from(RssSyndicationExt::Table, RssSyndicationExt::ChannelId)
                            .to(RssChannel::Table, RssChannel::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RssCategory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssChannel::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssCloud::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssDublinCoreExt::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssImage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssItem::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssItemEnclosure::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssItemGuid::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(RssItunesItemExtension::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(RssSyndicationExt::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RssTextInput::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum RssChannel {
    CreatedAt,
    UpdatedAt,
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
    Generator,
    Docs,
    Rating,
    Ttl,
    SkipHours,
    SkipDays,
    Extensions,
    Namespaces,
}

#[derive(DeriveIden)]
enum RssItem {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    Title,
    Link,
    Description,
    Author,
    Comments,
    PubDate,
    Content,
    Extensions,
}

#[derive(DeriveIden)]
enum RssCategory {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    ItemId,
    Name,
    Domain,
}

#[derive(DeriveIden)]
enum RssItemEnclosure {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ItemId,
    Url,
    Length,
    MimeType,
}

#[derive(DeriveIden)]
enum RssItemGuid {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ItemId,
    Value,
    Permalink,
}

#[derive(DeriveIden)]
enum RssItunesItemExtension {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ItemId,
    Author,
    Block,
    Image,
    Duration,
    Explicit,
    ClosedCaption,
    Order,
    Subtitle,
    Summary,
    Keywords,
    Episode,
    Season,
    EpisodeType,
}

#[derive(DeriveIden)]
enum RssDublinCoreExt {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ItemId,
    Contributors,
    Coverages,
    Creators,
    Dates,
    Descriptions,
    Formats,
    Identifiers,
    Languages,
    Publishers,
    Relations,
    Rights,
    Sources,
    Subjects,
    Titles,
    Types,
}

#[derive(DeriveIden)]
enum RssCloud {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    Domain,
    Port,
    Path,
    RegisterProcedure,
    Protocol,
}

#[derive(DeriveIden)]
enum RssImage {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    Url,
    Title,
    Link,
    Width,
    Height,
    Description,
}

#[derive(DeriveIden)]
enum RssTextInput {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    Title,
    Description,
    Name,
    Link,
}

#[derive(DeriveIden)]
enum RssSyndicationExt {
    CreatedAt,
    UpdatedAt,
    Table,
    Id,
    ChannelId,
    Period,
    Frequency,
    Base,
}
