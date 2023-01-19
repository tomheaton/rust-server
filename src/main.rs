#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

struct Response<T> {
    success: bool,
    message: String,
    data: T,
}

struct User {
    id: i32,
    username: String,
}

#[get("/")]
fn index() -> &'static str {
    return "tomheaton";
}

/*#[get("/user", format = "json")]
fn user() -> User {
    return User {
        id: 1,
        username: "tomheaton".to_string(),
    };
}*/

#[get("/")]
fn world() -> &'static str {
    return "Hello, World!";
}

#[get("/<name>")]
// fn hello(name: &str) -> String {
fn hello(name: &str) -> Json<String> {
    return format!("Hello, {}!", name);
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        // .mount("/", routes![index, user])
        .mount("/", routes![index])
        .mount("/hello", routes![hello, world])
        .launch()
        .await?;

    return Ok(());
}
