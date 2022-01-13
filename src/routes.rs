use super::db::Conn as DbConn;
use super::models::{NewUser, User};
use rocket_contrib::json::Json;
use serde_json::Value;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;


#[get("/users")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Result<status::Created<Json<User>>, Status> {
    User::insert_user(new_user.into_inner(), &conn)
    .map(|user| user_created(user))
    .map_err(|error| error_status(error))
}

#[get("/users/<id>")]
pub fn find_user(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_id(id, &conn),
    }))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

fn user_created(user: User) -> status::Created<Json<User>> {
    println!("here final");
    status::Created(
        format!("api/v1/users").to_string(),
        Some(Json(user)))
}