use crate::models::movies::Movie;
use rocket::serde::json::Json;

const base_url: &str = "http://www.omdbapi.com/";
const api_key: &str = "&apikey=353d840a";

#[get("/movie/title/<title>")]
pub async fn get_by_title(title: String) -> Json<Movie> {
    let url = format!("http://www.omdbapi.com/?t={}{}", title, api_key);
    let result = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Movie>()
        .await
        .unwrap();
    Json(result)
}

#[get("/movie/id/<id>")]
pub async fn get_by_id(id: String) -> Json<Movie> {
    let url = format!("http://www.omdbapi.com/?i={}{}", id, api_key);
    let result = reqwest::get(&url)
        .await
        .unwrap()
        .json::<Movie>()
        .await
        .unwrap();
    Json(result)
}
