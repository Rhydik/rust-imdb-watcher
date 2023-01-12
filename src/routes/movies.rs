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

//write a scraper that gets the top 250 movies from IMDB
//and returns a list of movies with their id, title and rating

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
            .find(Class(r#"ratingColumn"#).descendant(Name("div")).descendant(Class("inline")).descendant(Class("seen")))
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

// async fn get_top250() -> Result<Vec<Movies>, reqwest::Error> {
//     let res = reqwest::get(TOP250_URL).await?;
//     let mut result = Vec::new();

//     let html = res.text().await?;

//     let scraper = Html::parse_document(&html);

//     let title_selector = Selector::parse(r#"td.titleColumn"#).unwrap();
//     let rating_selector = Selector::parse(r#"td.ratingColumn"#).unwrap();
//     let mut id = 0;
//     //get the title and rating of each movie

//     let mut title_list = Vec::new();
//     let mut rating_list = Vec::new();

//     for element in scraper.select(&title_selector) {
//         let title = element.select(&Selector::parse(r#"a"#).unwrap()).next().unwrap().inner_html();
//         title_list.push(title);
//     }

//     for element in scraper.select(&rating_selector) {

//         let rating = element.select(&Selector::parse(r#"strong"#).unwrap()).next().unwrap().inner_html();
//         //create a list of ratings
//         rating_list.push(rating);

//         rating_list.push(rating);
//     }

//     //take title list and rating list and create a list of movies
//     for i in 0..title_list.len() {
//         id += 1;
//         let movie = Movies {
//             id,
//             title: title_list[i].to_string(),
//             rating: rating_list[i].to_string(),
//         };
//         result.push(movie);
//     }

//     Ok(result)
// }

#[get("/movies")]
pub async fn get_movies() -> Json<Vec<Movies>> {
    let movies = get_top250().await.unwrap();
    Json(movies)
}
