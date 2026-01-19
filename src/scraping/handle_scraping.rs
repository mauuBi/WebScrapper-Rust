use scraper::{self, Html, Selector};
use sqlx::{SqlitePool};
use crate::{db_rust::handle_database};


#[derive(Debug, sqlx::FromRow)] 
pub struct NewArticle {
    pub title: String,
    pub authors: Vec<String>,
    pub url: String,
	pub topic: String,
}

pub async fn parse_html(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>>{

	let body = get_html().await?;

    let document = Html::parse_document(&body);
    let article_selector = Selector::parse("li.wp-block-post").unwrap();
    let title_selector = Selector::parse("h3.loop-card__title").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let topic_selector = Selector::parse("div.loop-card__cat-group").unwrap();
    let author_selector = Selector::parse("a.loop-card__author").unwrap();
    let mut authors: Vec<String> = Vec::new();

    for article in document.select(&article_selector){
        let title = article.select(&title_selector).next().map(|el |el.text().collect::<String>());

        let link = article.select(&link_selector).next().and_then(|el| el.value().attr("href"));

        for author_element in article.select(&author_selector){
            let new_author = author_element.text().collect::<String>();
            if !authors.contains(&new_author){
                    authors.push(new_author);
            }
        }
        let topic = article.select(&topic_selector).next().map(|el| el.text().collect::<String>());
        println!(
            "Article: {}\nPar: {:?}\nSujet: {}\nLien: {}\n---",
            title.clone().unwrap_or_default().trim(),
            authors,
            topic.clone().unwrap_or_default().trim(),
            link.unwrap_or_default().trim()
        );
		let new_article = NewArticle{
			title: title.unwrap_or_default(),
			authors: authors.clone(),
			url: String::from(link.unwrap_or_default()),
			topic: topic.unwrap_or_default(),
		};

		handle_database::save_article(pool, new_article).await?;
        authors.clear();
    }
	Ok(())
}

async fn get_html() -> Result<String, Box<dyn std::error::Error>> {
	let my_client = reqwest::Client::new(); //Creation d'un client.
    let my_request = my_client.get("https://techcrunch.com/category/startups/").build()?; //Creation d'une requete.
    let body = my_client.execute(my_request)
    .await?
    .text()
    .await?; //Le client execute la requete.

	Ok(body)
}