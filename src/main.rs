use anyhow::Result;
use rss::Channel;
use rusqlite::Connection;
use thiserror::Error;

const RSS_URL: &str = "https://trevor-barnes.com/feed";

#[derive(Debug, Error)]
pub enum RssLiteError {
    #[error("SQLite Connection Error")]
    ConnectionError(#[from] rusqlite::Error),
}

#[derive(Debug)]
pub struct RSSChannel {
    pub title: String,
    pub link: String,
    pub description: String,
    pub language: Option<String>,
    pub copyright: Option<String>,
    pub managing_editor: Option<String>,
    pub webmaster: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub categories: Vec<rss::Category>,
    pub generator: Option<String>,
    pub docs: Option<String>,
    pub cloud: Option<rss::Cloud>,
    pub rating: Option<String>,
    pub ttl: Option<String>,
    pub image: Option<rss::Image>,
    pub text_input: Option<rss::TextInput>,
    pub skip_hours: Vec<String>,
    pub skip_days: Vec<String>,
    pub items: Vec<rss::Item>,
    pub extensions: rss::extension::ExtensionMap,
    pub itunes_ext: Option<rss::extension::itunes::ITunesChannelExtension>,
    pub dublin_core_ext: Option<rss::extension::dublincore::DublinCoreExtension>,
    pub syndication_ext: Option<rss::extension::syndication::SyndicationExtension>,
    pub namespaces: std::collections::BTreeMap<String, String>,
}

pub enum ConnectionType {
    InMemory,
    FromPath(String),
}

async fn init_db(connection_type: ConnectionType) -> Result<Connection> {
    let conn = match connection_type {
        ConnectionType::InMemory => Connection::open_in_memory(),
        ConnectionType::FromPath(path) => Connection::open(path),
    };

    Ok(conn.unwrap_or(Connection::open_in_memory()?))
}

async fn write_feed_to_db(conn: Connection, feed: &Channel) -> Result<()> {
    conn.execute(
        "INSERT INTO rss_feed (title, link, description, etc...) VALUES (?1, ?2, ?3, 4?...)",
        (
            feed.title.clone(),
            feed.link.clone(),
            feed.description.clone(),
            "etc",
        ),
    )?;

    Ok(())
}

async fn read_feed(url: &str) -> Result<Channel> {
    let feed = reqwest::get(url).await?.bytes().await?;

    let channel = Channel::read_from(&feed[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<()> {
    let feed = read_feed(RSS_URL).await?;

    let conn = init_db(ConnectionType::InMemory).await?;
    write_feed_to_db(conn, &feed).await?;

    println!("{:#?}", feed);
    Ok(())
}


#[cfg(test)]
pub mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    
    #[tokio::test]
    async fn init_db_no_path() {
        let connection_type = ConnectionType::InMemory;
        let db = init_db(connection_type).await.unwrap();
        assert_eq!(db.close().unwrap(), ());
    }
}