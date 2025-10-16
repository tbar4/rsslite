use anyhow::Result;
use models::rss::rss_channel::Entity as RssChannel;
use models::rss::rss_channel::{self, ActiveModel, Entity, Model};
use reqwest::Url;
use rss::Channel;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::Value;

async fn read_channel(url: impl ToString) -> Result<Channel> {
    let feed = reqwest::get(url.to_string()).await?.bytes().await?;

    let channel = Channel::read_from(&feed[..])?;
    Ok(channel)
}

/// Unified loader key – either an integer primary key or a channel URL.
pub enum LoadKey {
    Id(i32),
    Url(String),
}

/// Load a single RSS channel entry from the database using either its `id` or its `url`.
async fn load_item(
    db: &DatabaseConnection,
    key: &LoadKey,
) -> Result<Option<Model>, sea_orm::DbErr> {
    match key {
        LoadKey::Id(id) => RssChannel::find_by_id(*id).one(db).await,
        LoadKey::Url(url) => {
            RssChannel::find()
                .filter(rss_channel::Column::Link.eq(url))
                .one(db)
                .await
        }
    }
}

pub async fn add(db: &DatabaseConnection, url: LoadKey) -> Result<()> {
    if let None = load_item(db, &url).await.unwrap() {
        let url: Option<String> = match url {
            LoadKey::Id(_) => None,
            LoadKey::Url(url) => Some(url.clone()),
        };
        let channel = read_channel(url.unwrap()).await?;

        let item = rss_channel::ActiveModel {
            title: Set(channel.title),
            link: Set(channel.link),
            description: Set(channel.description),
            language: Set(channel.language),
            copyright: Set(channel.copyright),
            managing_editor: Set(channel.managing_editor),
            webmaster: Set(channel.webmaster),
            pub_date: Set(channel.pub_date),
            last_build_date: Set(channel.last_build_date),
            generator: Set(channel.generator),
            docs: Set(channel.docs),
            rating: Set(channel.rating),
            ttl: Set(channel.ttl),
            skip_hours: Set(serde_json::json!(channel.skip_hours)),
            skip_days: Set(serde_json::json!(channel.skip_days)),
            extensions: Set(serde_json::json!(channel.extensions)),
            namespaces: Set(Some(serde_json::json!(channel.namespaces))),
            ..Default::default()
        };

        let last_id = RssChannel::insert(item).exec(db).await?.last_insert_id;
    }

    Ok(())
}
