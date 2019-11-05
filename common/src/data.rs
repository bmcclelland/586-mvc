use serde::{Serialize,Deserialize};
use failure::{Fail,Error};

pub trait Inc {
    fn inc(&mut self);
}

#[derive(Serialize,Deserialize,Fail,Debug)]
pub enum AppError {
    #[fail(display = "not found")]
    NotFound,
}

pub type AppResult<T> = Result<T,Error>;

newtype!(WorkerName:  String);
newtype!(TaskName:    String);
newtype!(ProjectName: String);

newtype_id!(WorkerID:    i32);
newtype_id!(TaskID:      i32);
newtype_id!(ProjectID:   i32);

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize)]
pub struct Project {
    pub project_id:   ProjectID,
    pub project_name: ProjectName,
}

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize)]
pub struct Task {
    pub task_id:    TaskID,
    pub task_name:  TaskName,
    pub project_id: ProjectID,
}

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize)]
pub struct Worker {
    pub worker_id:   WorkerID,
    pub worker_name: WorkerName,
}

#[derive(Debug,Clone,PartialEq,Queryable,Serialize,Deserialize)]
pub struct WorkerTask {
    pub worker_id: WorkerID,
    pub task_id:   TaskID,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddProjectParams {
    pub project_name: ProjectName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddWorkerParams {
    pub worker_name: WorkerName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddTaskParams {
    pub task_name: TaskName,
    pub project_id: ProjectID,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetTasksParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetProjectsParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetWorkersParams;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DeleteProjectParams {
    pub project_id: ProjectID,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct DeleteWorkerParams {
    pub worker_id: WorkerID,
}
