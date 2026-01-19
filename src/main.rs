use tokio;
use sqlx::{SqlitePool};
mod scraping;
mod db_rust;


use crate::{db_rust::handle_database, scraping::handle_scraping::parse_html};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite:scraper.db?mode=rwc").await?;
    handle_database::setup_database(&pool);
    parse_html(&pool).await?;

    Ok(())
}