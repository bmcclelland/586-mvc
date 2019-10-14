use crate::schema::*;
use common::*;

#[derive(Debug,Insertable)]
#[table_name="workertasks"]
pub struct NewWorkerTask {
    pub worker_id: WorkerID,
    pub task_id:   TaskID,
}

#[derive(Debug,Insertable)]
#[table_name="workers"]
pub struct NewWorker {
    pub worker_name: WorkerName,
}

#[derive(Debug,Insertable)]
#[table_name="tasks"]
pub struct NewTask {
    pub task_name:  TaskName,
    pub project_id: ProjectID,
}

#[derive(Debug,Insertable)]
#[table_name="projects"]
pub struct NewProject {
    pub project_name: ProjectName,
}
