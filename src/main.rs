use anyhow::Result;
use dotenv::dotenv;
use rss::{Category, Channel, Enclosure, Item, Source};
use sea_orm::{
    Database, DatabaseConnection, FromJsonQueryResult, entity::*, sea_query::OnConflict,
};
use thiserror::Error;

use models::rss::{rss_channel, rss_item};

#[derive(Debug, Error)]
pub enum RssLiteError {
    #[error("SeaORM Connection Error")]
    ConnectionError(#[from] sea_orm::error::ConnAcquireErr),
    #[error("Environment Variable Error")]
    EnvError(#[from] dotenv::Error),
}

const RSS_URL: &str = "https://trevor-barnes.com/feed";

pub enum ConnectionType {
    InMemory,
    FromPath(String),
}

async fn init_db(connection_type: ConnectionType) -> Result<DatabaseConnection> {
    let conn = match connection_type {
        ConnectionType::InMemory => Database::connect("sqlite::memory:").await?,
        ConnectionType::FromPath(path) => Database::connect(path).await?,
    };

    Ok(conn)
}

async fn read_feed(url: &str) -> Result<Channel> {
    let feed = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&feed[..])?;
    Ok(channel)
}

async fn upsert_feed_to_db(conn: DatabaseConnection, feed: Channel) -> Result<()> {
    for f in feed.items() {
        let categories = Some(
            f.categories
                .iter()
                .map(|c| c.name.clone())
                .collect::<Vec<String>>()
                .join(", "),
        );

        let feed_item = rss_item::ActiveModel {
            title: Set(f.title.clone()),
            link: Set(f.link.clone().unwrap()),
            description: Set(f.description.clone()),
            author: Set(f.author.clone()),
            categories: Set(categories),
            comments: Set(f.comments.clone()),
            enclosure_url: Set(Some(
                f.enclosure.clone().unwrap_or(Enclosure::default()).url,
            )),
            enclosure_length: Set(Some(
                f.enclosure.clone().unwrap_or(Enclosure::default()).length,
            )),
            enclosure_mime_type: Set(Some(
                f.enclosure
                    .clone()
                    .unwrap_or(Enclosure::default())
                    .mime_type,
            )),
            guid: Set(Some(f.guid.clone().unwrap().value)),
            pub_date: Set(f.pub_date.clone()),
            source_title: Set(f.source.clone().unwrap_or(Source::default()).title),
            source_url: Set(Some(f.source.clone().unwrap_or(Source::default()).url)),
            content: Set(f.content.clone()),
            ..Default::default()
        };

        rss_item::Entity::insert(feed_item)
            .on_conflict(
                OnConflict::column(rss_item::Column::Link)
                    .update_columns([
                        rss_item::Column::Title,
                        rss_item::Column::Description,
                        rss_item::Column::Author,
                        rss_item::Column::Categories,
                        rss_item::Column::Comments,
                        rss_item::Column::EnclosureUrl,
                        rss_item::Column::EnclosureLength,
                        rss_item::Column::EnclosureMimeType,
                        rss_item::Column::Guid,
                        rss_item::Column::PubDate,
                        rss_item::Column::SourceTitle,
                        rss_item::Column::SourceUrl,
                        rss_item::Column::Content,
                    ])
                    .to_owned(),
            )
            .exec(&conn)
            .await?;

        println!("{:#?}", f);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL")?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let feed = read_feed(RSS_URL).await?;
    println!("{:#?}", feed);
    let conn = init_db(ConnectionType::FromPath(db_url)).await?;
    upsert_feed_to_db(conn, feed).await?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use super::*;
    //use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn valid_db_connection() {
        let db = init_db(ConnectionType::InMemory).await.unwrap();
        assert!(db.ping().await.is_ok());
    }
}
