CREATE TABLE tasks (
    task_id INTEGER PRIMARY KEY NOT NULL,
    task_name TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) 
        REFERENCES projects(project_id)
        ON DELETE CASCADE
)
