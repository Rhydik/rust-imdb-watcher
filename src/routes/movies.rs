use reqwest;
use rocket::serde::{json::Json, Deserialize, Serialize};
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
const TOP250_URL: &str = "http://www.imdb.com/chart/top";

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Movies {
    id: usize,
    title: String,
    rating: String,
}

async fn get_top250() -> Result<Vec<Movies>, reqwest::Error> {
    let res = reqwest::get(TOP250_URL).await?;
    let mut result = Vec::new();
    let document = Document::from(res.text().await?.as_str());
    let mut id = 0;
    for node in document.find(Class("lister-list").descendant(Name("tr"))) {
        let title = node
            .find(Class("titleColumn").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        let rating = node
            .find(
                Class(r#"ratingColumn"#)
                    .descendant(Name("div"))
                    .descendant(Class("inline"))
                    .descendant(Class("seen")),
            )
            .next()
            .unwrap()
            .text();
        let movie = Movies {
            id: id,
            title: title.to_string(),
            rating: rating.to_string(),
        };
        result.push(movie);
        id += 1;
    }

    Ok(result)
}

#[get("/movies")]
pub async fn get_movies() -> Json<Vec<Movies>> {
    let movies = get_top250().await.unwrap();
    Json(movies)
}
