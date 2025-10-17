use super::rss_item::add_item;
use anyhow::Result;
use models::rss::rss_channel::{self};
use models::rss::rss_channel::{Column, Entity as RssChannel};
use rss::Channel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing;

async fn read_channel(url: impl ToString) -> Result<Channel> {
    let feed = reqwest::get(url.to_string()).await?.bytes().await?;

    let channel = Channel::read_from(&feed[..])?;
    Ok(channel)
}

pub async fn add_channel(db: &DatabaseConnection, url: &str) -> Result<()> {
    tracing::info!("Fetching RSS channel from {}", url);
    let channel = read_channel(url).await?;
    tracing::debug!("Fetched {} items", channel.items.len());
    let channel_clone = channel.clone();
    let link_clone = channel_clone.link();
    let description_clone = channel_clone.description();

    let channel_model = rss_channel::ActiveModel {
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

    let channel_dupe = RssChannel::find()
        .filter(
            Column::Link
                .eq(link_clone)
                .and(Column::Description.eq(description_clone)),
        )
        .one(db)
        .await?;

    if let Some(chan) = channel_dupe {
        let channel_id = chan.id;
        tracing::info!("Channel already exists...\nAdding feed items...");
        add_item(db, channel_clone, channel_id).await?;
        tracing::info!("Done adding feed items...");
    } else {
        tracing::info!("Channel Not Found...\nAdding Channel to DB...");
        let channel_id = RssChannel::insert(channel_model)
            .exec(db)
            .await?
            .last_insert_id;
        tracing::info!("Adding feed items...");
        add_item(db, channel_clone, channel_id).await?;
        tracing::info!("Done adding feed items...");
    }

    Ok(())
}
