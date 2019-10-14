CREATE TABLE workertasks (
    worker_id INTEGER NOT NULL,
    task_id INTEGER NOT NULL,
    FOREIGN KEY(worker_id) 
        REFERENCES workers(worker_id) 
        ON DELETE CASCADE,
    FOREIGN KEY(task_id) 
        REFERENCES tasks(task_id) 
        ON DELETE CASCADE,
    PRIMARY KEY(worker_id, task_id) 
        ON CONFLICT IGNORE
)
