use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let key = "DATABASE_URL";
    if std::env::var(key).is_err() {
        std::env::set_var("DATABASE_URL", "sqlite::memory:");
    }
    cli::run_cli(migration::Migrator).await;
}
