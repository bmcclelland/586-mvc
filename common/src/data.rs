use serde::Serialize;
use serde::Deserialize;

pub trait Inc {
    fn inc(&mut self);
}

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
pub struct AddProjectPayload {
    pub project_name: ProjectName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddWorkerPayload {
    pub worker_name: WorkerName,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AddTaskPayload {
    pub task_name: TaskName,
    pub project_id: ProjectID,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetTasksPayload;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetProjectsPayload;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct GetWorkersPayload;
