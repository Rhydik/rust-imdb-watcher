use users::Users;
use movies::Movies;

struct Rating {
    movie_id: i32,
    rating: i32,
}

impl Rating {
    fn new(movie_id: i32, rating: i32) -> Rating {
        Rating {
            movie_id,
            rating,
        }
    }
}