CREATE TABLE users_tasks
(
    user_id INT NOT NULL,
    task_id INT NOT NULL,
    CONSTRAINT PK_UsersTasks PRIMARY KEY(user_id, task_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE
)