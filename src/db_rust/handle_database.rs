use sqlx::SqlitePool;
use crate::{scraping::handle_scraping};


pub async fn save_article(pool: &SqlitePool, article: handle_scraping::NewArticle) ->  Result<(), Box<dyn std::error::Error>> {
    sqlx::query("INSERT INTO articles (title, authors, url, topic) VALUES (?, ?, ?, ?)")
        .bind(article.title)
        .bind(sqlx::types::Json(article.authors))
        .bind(article.url)
		.bind(article.topic)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn setup_database(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS articles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            authors TEXT NOT NULL,
            url TEXT NOT NULL,
			topic TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    Ok(())
}