use thiserror::Error;

#[derive(Debug, Error)]
pub enum RssLiteError {
    #[error("SeaORM Connection Error")]
    ConnectionError(#[from] sea_orm::error::ConnAcquireErr),
    #[error("Environment Variable Error")]
    EnvError(#[from] dotenv::Error),
}