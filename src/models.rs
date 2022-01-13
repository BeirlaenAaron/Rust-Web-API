use super::schema::users;
use super::schema::users::dsl::users as all_users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
#[derive(Serialize, Queryable)]
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

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(conn)
    }

    pub fn get_user_by_id(id: i32, conn: &PgConnection) -> Vec<User> {
        all_users
            .filter(users::id.eq(id))
            .load::<User>(conn)
            .expect("error!")
    }
}
