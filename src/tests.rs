use super::rocket;
use rocket::local::Client;
use super::models::{User, UserTask, Task};
use rocket::http::{Status, ContentType};
use rocket::http::Header;

static API_KEY: &'static str = "eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6ImFkbWluIiwiYXVkIjpudWxsLCJleHAiOm51bGwsIm5iZiI6bnVsbCwiaWF0IjpudWxsLCJqdGkiOm51bGx9.7ym4VwEhMNyhL1jCH8jlr/e4ADgf2vRqU9kCeK70JBU";

#[test]
fn test_01_get_users() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api/v1/users").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().unwrap();
    assert!(response_body.contains("result"));
}

#[test]
fn test_02_get_tasks() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api/v1/tasks").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().unwrap();
    assert!(response_body.contains("result"));
}

#[test]
fn test_03_new_user(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/api/v1/users")
        .header(ContentType::JSON)
        .body(r##"{
            "id": 2147483647,
            "username": "John Doe",
            "first_name": "John",
            "password": "123456"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let user: User = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.username, "John Doe");
    assert_eq!(user.first_name, "John");
}

#[test]
fn test_04_new_task(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/api/v1/tasks")
        .header(ContentType::JSON)
        .header(Header::new("Authentication", API_KEY))
        .body(r##"{
            "id": 2147483647,
            "description": "Een taak",
            "reward": 1000,
            "expiry_date": "2022-02-01T08:00:00"
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let user: Task = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(user.description, "Een taak");
    assert_eq!(user.reward, 1000);
}

#[test]
fn test_05_get_user_by_id() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api/v1/users/2147483647").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().unwrap();
    assert!(response_body.contains("John Doe"));
}

#[test]
fn test_06_get_task_by_id() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/api/v1/tasks/2147483647").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().unwrap();
    assert!(response_body.contains("Een taak"));
}

#[test]
fn test_07_new_assignment(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.post("/api/v1/users/tasks")
        .header(ContentType::JSON)
        .header(Header::new("Authentication", API_KEY))
        .body(r##"{
            "user_id": 2147483647,
            "task_id": 2147483647
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    let response_body = response.body_string().expect("Response Body");
    let assignment: UserTask = serde_json::from_str(&response_body.as_str()).expect("Valid User Response");
    assert_eq!(assignment.user_id, 2147483647);
    assert_eq!(assignment.task_id, 2147483647);
}

#[test]
fn test_08_unauthorized_delete_user(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.delete("/api/v1/users/2147483647").dispatch();
    assert_eq!(response.status(), Status::Unauthorized);
}

#[test]
fn test_09_delete_user(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.delete("/api/v1/users/2147483647")
        .header(Header::new("Authentication", API_KEY))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_10_delete_task(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.delete("/api/v1/tasks/2147483647")
        .header(Header::new("Authentication", API_KEY))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn test_11_fail_new_assignment(){
    let client = Client::new(rocket()).expect("valid rocket instance");
    let response = client.post("/api/v1/users/tasks")
        .header(ContentType::JSON)
        .header(Header::new("Authentication", API_KEY))
        .body(r##"{
            "user_id": 2147483647,
            "task_id": 2147483647
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::BadRequest);
}
