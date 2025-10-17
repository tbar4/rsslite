use anyhow::Result;
use sea_orm::{DatabaseConnection, Database, ConnectOptions};

pub enum ConnectionType {
    InMemory,
    FromPath(String),
}

pub async fn init_db(connection_type: ConnectionType) -> Result<DatabaseConnection> {
    let conn = match connection_type {
        ConnectionType::InMemory => {
            let mut opt = ConnectOptions::new("sqlite::memory:").to_owned();
            opt.sqlx_logging(false);
            Database::connect(opt).await?
        }
        ConnectionType::FromPath(path) => {
            let mut opt = ConnectOptions::new(path).to_owned();
            opt.sqlx_logging(false);
            Database::connect(opt).await?
        }
    };

    Ok(conn)
}