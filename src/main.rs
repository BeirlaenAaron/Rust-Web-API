#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use routes::*;
use std::env;
use rocket_okapi::{routes_with_openapi};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod db;
mod models;
mod routes;
mod schema;
mod auth;
#[cfg(test)] mod tests;

fn get_docs() -> SwaggerUIConfig {
    use rocket_okapi::swagger_ui::UrlObject;
 
    SwaggerUIConfig {
        url: "/api/v1/openapi.json".to_string(),
        urls: vec![UrlObject::new("Swagger Docs", "/api/v1/openapi.json")],
        ..Default::default()
    }
}

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes_with_openapi![get_users, new_user, find_user, update_user, delete_user, get_tasks, new_task, find_task, update_task, update_task_status, delete_task, find_user_tasks, find_task_users, new_assignment, delete_assignment, update_user_unauthorized, delete_user_unauthorized, new_task_unauthorized, update_task_unauthorized, update_task_status_unauthorized, delete_task_unauthorized, new_assignment_unauthorized, delete_assignment_unauthorized])
        .mount("/auth/", routes![login])
        .mount("/swagger", make_swagger_ui(&get_docs()))
}

fn main() {
    rocket().launch();
}
