struct User {
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
}
