use super::db::Conn as DbConn;
use super::models::{NewUser, User, NewTask, Task, TaskStatus, UserTask};
use rocket_contrib::json::Json;
use serde_json::Value;
use diesel::result::Error;
use rocket::http::Status;
use rocket_okapi::openapi;

#[openapi]
#[get("/users")]
pub fn get_users(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[openapi]
#[get("/users/<id>")]
pub fn find_user(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_id(id, &conn),
    }))
}

#[openapi(skip)]
#[post("/users", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Result<Json<User>, Status> {
    User::insert_user(new_user.into_inner(), &conn)
    .map(|user| Json(user))
    .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[put("/users/<id>", format = "application/json", data = "<user>")]
pub fn update_user(id: i32, user: Json<NewUser>, connection: DbConn) -> Result<Json<User>, Status> {
    User::update_user(id, user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[delete("/users/<id>")]
pub fn delete_user(conn: DbConn, id: i32) -> Result<Status, Status> {
    User::delete_user(id, &conn)
        .map(|_| Status::Ok)
        .map_err(|error| error_status(error))
}

#[openapi]
#[get("/tasks")]
pub fn get_tasks(conn: DbConn) -> Json<Value> {
    let tasks = Task::get_all_tasks(&conn);
    Json(json!({
        "status": 200,
        "result": tasks,
    }))
}

#[openapi]
#[get("/tasks/<id>")]
pub fn find_task(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Task::get_task_by_id(id, &conn),
    }))
}

#[openapi(skip)]
#[post("/tasks", format = "application/json", data = "<new_task>")]
pub fn new_task(conn: DbConn, new_task: Json<NewTask>) -> Result<Json<Task>, Status> {
    Task::insert_task(new_task.into_inner(), &conn)
    .map(|task| Json(task))
    .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[put("/tasks/<id>", format = "application/json", data = "<task>")]
pub fn update_task(id: i32, task: Json<NewTask>, connection: DbConn) -> Result<Json<Task>, Status> {
    Task::update_task(id, task.into_inner(), &connection)
        .map(|task| Json(task))
        .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[put("/tasks/<id>/status", format = "application/json", data = "<task>")]
pub fn update_task_status(id: i32, task: Json<TaskStatus>, connection: DbConn) -> Result<Json<Task>, Status> {
    Task::update_task_status(id, task.into_inner(), &connection)
        .map(|task| Json(task))
        .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[delete("/tasks/<id>")]
pub fn delete_task(conn: DbConn, id: i32) -> Result<Status, Status> {
    Task::delete_task(id, &conn)
        .map(|_| Status::Ok)
        .map_err(|error| error_status(error))
}

#[openapi]
#[get("/users/<id>/tasks")]
pub fn find_user_tasks(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": UserTask::get_user_tasks(id, &conn),
    }))
}

#[openapi(skip)]
#[post("/users/tasks", format = "application/json", data = "<new_assignment>")]
pub fn new_assignment(conn: DbConn, new_assignment: Json<UserTask>) -> Result<Json<UserTask>, Status> {
    UserTask::insert_assignment(new_assignment.into_inner(), &conn)
    .map(|assignment| Json(assignment))
    .map_err(|error| error_status(error))
}

#[openapi(skip)]
#[delete("/users/tasks", format = "application/json", data = "<assignment>")]
pub fn delete_assignment(conn: DbConn, assignment: Json<UserTask>) -> Result<Status, Status> {
    UserTask::delete_assignment(assignment.into_inner(), &conn)
        .map(|_| Status::Ok)
        .map_err(|error| error_status(error))
}

#[openapi]
#[get("/tasks/<id>/users")]
pub fn find_task_users(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": UserTask::get_task_users(id, &conn),
    }))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::BadRequest
    }
}