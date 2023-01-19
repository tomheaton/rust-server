#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "tomheaton";
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    return format!("Hello, {}!", name);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .launch()
        .await?;

    return Ok(());
}
