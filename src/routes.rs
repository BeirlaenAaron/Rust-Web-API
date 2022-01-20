use super::db::Conn as DbConn;
use super::models::{NewUser, User, NewTask, Task, UserTask};
use rocket_contrib::json::Json;
use serde_json::Value;
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;


#[get("/users")]
pub fn get_users(conn: DbConn) -> Json<Value> {
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

#[get("/tasks")]
pub fn get_tasks(conn: DbConn) -> Json<Value> {
    let tasks = Task::get_all_tasks(&conn);
    Json(json!({
        "status": 200,
        "result": tasks,
    }))
}

#[post("/tasks", format = "application/json", data = "<new_task>")]
pub fn new_task(conn: DbConn, new_task: Json<NewTask>) -> Result<status::Created<Json<Task>>, Status> {
    Task::insert_task(new_task.into_inner(), &conn)
    .map(|task| task_created(task))
    .map_err(|error| error_status(error))
}

#[get("/tasks/<id>")]
pub fn find_task(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Task::get_task_by_id(id, &conn),
    }))
}

#[get("/users/<id>/tasks")]
pub fn find_user_tasks(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": UserTask::get_user_tasks(id, &conn),
    }))
}

#[post("/users/tasks", format = "application/json", data = "<new_assignment>")]
pub fn new_assignment(conn: DbConn, new_assignment: Json<UserTask>) -> Result<status::Created<Json<UserTask>>, Status> {
    UserTask::insert_assignment(new_assignment.into_inner(), &conn)
    .map(|assignment| assignment_created(assignment))
    .map_err(|error| error_status(error))
}

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
        _ => Status::InternalServerError
    }
}

fn user_created(user: User) -> status::Created<Json<User>> {
    status::Created(
        format!("api/v1/users").to_string(),
        Some(Json(user)))
}

fn task_created(task: Task) -> status::Created<Json<Task>> {
    status::Created(
        format!("api/v1/tasks").to_string(),
        Some(Json(task)))
}

fn assignment_created(assignment: UserTask) -> status::Created<Json<UserTask>> {
    status::Created(
        format!("api/v1/users/tasks").to_string(),
        Some(Json(assignment)))
}