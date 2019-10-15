use common::*;
use crate::data::*;

pub trait Model: Send+Sync {
    fn add_project(&mut self, _: NewProject) -> ProjectID;
    fn add_worker(&mut self, _: NewWorker) -> WorkerID;
    fn add_task(&mut self, _: NewTask) -> TaskID;
    
    fn delete_project(&self, _: ProjectID) -> bool;
    fn delete_worker(&self, _: WorkerID) -> bool;
    
    fn get_projects(&self) -> Vec<Project>;
    fn get_workers(&self) -> Vec<Worker>;
    fn get_tasks(&self) -> Vec<Task>;
   
    fn get_project(&self, _: ProjectID) -> Option<Project>;
    fn get_worker(&self, _: WorkerID) -> Option<Worker>;
    fn get_task(&self, _: TaskID) -> Option<Task>;
   
    fn get_project_tasks(&self, _: ProjectID) -> Vec<Task>;
    fn get_worker_tasks(&self, _: WorkerID) -> Vec<Task>;

    fn assign_task(&mut self, _: NewWorkerTask);
    fn unassign_task(&mut self, _: NewWorkerTask);
}
