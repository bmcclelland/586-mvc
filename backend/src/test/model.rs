#![allow(unused_imports,dead_code,unused_variables)]

use crate::traits::Model;
use crate::data::*;
use common::*;
use crate::schema::*;
use crate::*;
use super::*;

impl Model for Env {
    fn add_project(&mut self, item: NewProject) -> ProjectID {
        self.projects.insert(item)
    }

    fn add_worker(&mut self, item: NewWorker) -> WorkerID {
        self.workers.insert(item)
    }
    
    fn add_task(&mut self, item: NewTask) -> TaskID {
        self.tasks.insert(item)
    }

    fn get_projects(&self) -> Vec<Project> {
        self.projects.list()
    }
    
    fn get_workers(&self) -> Vec<Worker> {
        self.workers.list()
    }
    
    fn get_tasks(&self) -> Vec<Task> {
        self.tasks.list()
    }
    
    fn get_project(&self, project_id: ProjectID) -> Option<Project> {
        self.projects.get(project_id).cloned()
    }

    fn get_worker(&self, worker_id: WorkerID) -> Option<Worker> {
        self.workers.get(worker_id).cloned()
    }

    fn get_task(&self, task_id: TaskID) -> Option<Task> {
        self.tasks.get(task_id).cloned()
    }
    
    fn get_project_tasks(&self, project_id: ProjectID) -> Vec<Task> {
        self.tasks.list().drain(..)
            .filter(|x| x.project_id == project_id)
            .collect()
    }

    fn get_worker_tasks(&self, worker_id: WorkerID) -> Vec<Task> {
        let assigned_worker = |task: &Task| {
            self.workertasks.contains(
                &(task.task_id, worker_id)
            )
        };

        self.tasks.list().drain(..)
            .filter(assigned_worker)
            .collect()
    }
    
    fn assign_task(&mut self, item: NewWorkerTask) {
        self.workertasks.insert((item.task_id, item.worker_id));
    }

    fn unassign_task(&mut self, item: NewWorkerTask) {
        self.workertasks.remove(&(item.task_id, item.worker_id));
    }
}
