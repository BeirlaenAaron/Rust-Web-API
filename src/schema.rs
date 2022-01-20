table! {
    tasks (id) {
        id -> Int4,
        description -> Varchar,
        status -> Varchar,
        created_date -> Timestamp,
        expiry_date -> Timestamp,
        reward -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
    }
}

table! {
    users_tasks (user_id, task_id) {
        user_id -> Int4,
        task_id -> Int4,
    }
}

joinable!(users_tasks -> tasks (task_id));
joinable!(users_tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
    users_tasks,
);
