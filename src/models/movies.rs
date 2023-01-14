use reqwest;
use rocket::serde::{json::Json, Deserialize, Serialize};
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Movie {
    imdbID: String,
    Title: String,
    imdbRating: String,
}

