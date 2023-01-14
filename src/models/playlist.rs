use super::movies::Movie;

struct Playlist {
    id: i32,
    name: String,
    description: String,
    movies: Vec<Movie>,
}

impl Playlist {
    fn new(id: i32, name: String, description: String) -> Playlist {
        Playlist {
            id,
            name,
            description,
            movies: Vec::new(),
        }
    }

    fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }

    fn create_playlist(name: String, description: String) -> Playlist {
        let id = 1;
        println!("Playlist created: {}", name);
        Playlist::new(id, name, description)
    }
}