CREATE TABLE workers (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    worker_id INTEGER,
    FOREIGN KEY (project_id) 
        REFERENCES projects(id)
        ON DELETE CASCADE
    FOREIGN KEY (worker_id) 
        REFERENCES workers(id)
        ON DELETE SET NULL
);
CREATE TABLE projects (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);
