use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Rating {
    movie_id: i32,
    rating: i32,
}

impl Rating {
    pub fn new(movie_id: i32, rating: i32) -> Rating {
        Rating {
            movie_id,
            rating,
        }
    }
}