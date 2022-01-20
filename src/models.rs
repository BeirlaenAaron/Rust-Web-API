use super::schema::users;
use super::schema::tasks;
use super::schema::users_tasks;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use chrono::prelude::*;
#[derive(Identifiable, Serialize, Queryable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Identifiable, Serialize, Queryable, Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: String,
    pub created_date: NaiveDateTime,
    pub expiry_date: NaiveDateTime,
    pub reward: i32,
}


#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
    pub reward: i32,
    pub expiry_date: NaiveDateTime,
}

#[derive(Serialize, Queryable, Debug, Associations, Insertable, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Task)]
#[table_name = "users_tasks"]
pub struct UserTask {
    pub user_id: i32,
    pub task_id: i32,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        users::table
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    
    pub fn get_user_by_id(id: i32, conn: &PgConnection) -> Vec<User> {
        users::table
        .filter(users::id.eq(id))
        .load::<User>(conn)
        .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
    }
}

impl Task {
    pub fn get_all_tasks(conn: &PgConnection) -> Vec<Task> {
        tasks::table
            .order(tasks::id.desc())
            .load::<Task>(conn)
            .expect("error!")
    }
    
    pub fn get_task_by_id(id: i32, conn: &PgConnection) -> Vec<Task> {
        tasks::table
            .filter(tasks::id.eq(id))
            .load::<Task>(conn)
            .expect("error!")
    }

    pub fn insert_task(task: NewTask, conn: &PgConnection) -> QueryResult<Task> {
        diesel::insert_into(tasks::table)
            .values(&task)
            .get_result(conn)
    }

}

impl UserTask {
    pub fn insert_assignment(assignment: UserTask, conn: &PgConnection) -> QueryResult<UserTask> {
        diesel::insert_into(users_tasks::table)
            .values(&assignment)
            .get_result(conn)
    }

    pub fn get_user_tasks(id: i32, conn: &PgConnection) -> Vec<Task> {
        users::table
            .inner_join(users_tasks::table.inner_join(tasks::table))
            .filter(users::id.eq(id))
            .select(tasks::all_columns)
            .load::<Task>(conn)
            .expect("error!")
    }

    pub fn get_task_users(id: i32, conn: &PgConnection) -> Vec<User> {
        tasks::table
            .inner_join(users_tasks::table.inner_join(users::table))
            .filter(tasks::id.eq(id))
            .select(users::all_columns)
            .load::<User>(conn)
            .expect("error!")
    }
}
