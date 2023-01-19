#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

struct Response<T> {
    success: bool,
    message: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
}

#[get("/")]
fn index() -> &'static str {
    return "tomheaton";
}

#[get("/user")]
fn user() -> Json<User> {
    let user = User {
        id: 1,
        username: "tomheaton".to_string(),
    };
    return Json(user);
}

#[get("/")]
fn world() -> &'static str {
    return "Hello, World!";
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    return format!("Hello, {}!", name);
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, user])
        .mount("/hello", routes![hello, world])
        .launch()
        .await?;

    return Ok(());
}
