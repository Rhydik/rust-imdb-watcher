use crate::models::users::User;
use rocket::serde::json::Json;

#[post("/users/<name>/<email>")]
pub async fn set_user(name: String, email: String) -> Json<User> {
    let user = User::create_user(name, email);
    Json(user)
}