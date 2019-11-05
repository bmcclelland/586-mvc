//use common::*;
//use crate::data::*;
//
//pub trait Model: Send+Sync {
//    fn add_project(&mut self, _: NewProject) -> AppResult<ProjectID>;
//    fn add_worker(&mut self, _: NewWorker) -> AppResult<WorkerID>;
//    fn add_task(&mut self, _: NewTask) -> AppResult<TaskID>;
//    
//    fn delete_project(&self, _: ProjectID) -> AppResult<()>;
//    fn delete_worker(&self, _: WorkerID) -> AppResult<()>;
//    
//    fn get_projects(&self) -> AppResult<Vec<Project>>;
//    fn get_workers(&self) -> AppResult<Vec<Worker>>;
//    fn get_tasks(&self) -> AppResult<Vec<Task>>;
//
//    fn get_project(&self, _: ProjectID) -> AppResult<Project>;
//    fn get_worker(&self, _: WorkerID) -> AppResult<Worker>;
//    fn get_task(&self, _: TaskID) -> AppResult<Task>;
//   
//    fn get_project_tasks(&self, _: ProjectID) -> AppResult<Vec<Task>>;
//    fn get_worker_tasks(&self, _: WorkerID) -> AppResult<Vec<Task>>;
//
//    fn assign_task(&mut self, _: NewWorkerTask) -> AppResult<()>;
//    fn unassign_task(&mut self, _: NewWorkerTask) -> AppResult<()>;
//}
