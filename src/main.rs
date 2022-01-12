#[macro_use] extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/yo", routes![index])
}