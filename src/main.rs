use anyhow::Result;
use controllers::rss::rss_channel::add_channel;
use dotenv::dotenv;
use tokio::task;
use utils::db::{ConnectionType, init_db};
//use utils::db_logger::DbWriter;
use std::sync::Arc;

const RSS_URLS: &[&str] = &[
    "https://trevor-barnes.com/rss",
    "https://spacenews.com/feed",
    "https://hnrss.org/newest?q=rust",
    "https://hnrss.org/newcomments?q=rust",
    "https://hnrss.org/frontpage",
    "https://hnrss.org/bestcomments",
    "https://hnrss.org/whoishiring/jobs?q=rust",
];

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = dotenv::var("DATABASE_URL")?;
    let db = Arc::new(init_db(ConnectionType::FromPath(db_url)).await?);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut handles = Vec::new();

    for url in RSS_URLS {
        let db_clone = db.clone();
        let handle = task::spawn(async move { 
            tracing::debug!("Starting pull of {url} feed...");
            add_channel(&db_clone, url).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??
    }

    Ok(())
}
