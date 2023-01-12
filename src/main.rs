#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate select;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![routes::movies::get_movies])
        .launch()
        .await;

    Ok(())
}
