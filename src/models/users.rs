use reqwest;
use super::ratings::Rating;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    ratings: Option<Vec<Rating>>,
}

impl User {
    fn new(id: i32, name: String, email: String) -> User {
        User {
            id,
            name,
            email,
            ratings: None,
        }
    }

    fn rate_movie(&mut self, movie_id: i32, rating: i32) {
        let rating = Rating::new(movie_id, rating);
        match &mut self.ratings {
            Some(ratings) => ratings.push(rating),
            None => self.ratings = Some(vec![rating]),
        }
    }

    pub fn create_user(name: String, email: String) -> User {
        let id = 1;
        println!("User created: {}", name);
        User::new(id, name, email)
    }
}

