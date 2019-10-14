table! {
    projects (project_id) {
        project_id -> Integer,
        project_name -> Text,
    }
}

table! {
    tasks (task_id) {
        task_id -> Integer,
        task_name -> Text,
        project_id -> Integer,
    }
}

table! {
    workers (worker_id) {
        worker_id -> Integer,
        worker_name -> Text,
    }
}

table! {
    workertasks (worker_id, task_id) {
        worker_id -> Integer,
        task_id -> Integer,
    }
}

joinable!(tasks -> projects (project_id));
joinable!(workertasks -> tasks (task_id));
joinable!(workertasks -> workers (worker_id));

allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
    workers,
    workertasks,
);
