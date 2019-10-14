#![allow(unused_imports,dead_code,unused_variables)]

use crate::data::*;
use common::*;
use std::collections::{HashMap, HashSet};

pub trait DBType : Clone {
    type Key: Eq+std::hash::Hash+Copy+Inc;
    type New;

    fn make(new: &Self::New, key: &Self::Key) -> Self;
}

impl DBType for Project {
    type Key = ProjectID;
    type New = NewProject;
    fn make(new: &NewProject, key: &ProjectID) -> Project {
        Project {
            project_id: *key,
            project_name: new.project_name.clone(),
        }
    }
}

impl DBType for Worker {
    type Key = WorkerID;
    type New = NewWorker;
    fn make(new: &NewWorker, key: &WorkerID) -> Worker {
        Worker {
            worker_id: *key,
            worker_name: new.worker_name.clone(),
        }
    }
}

impl DBType for Task {
    type Key = TaskID;
    type New = NewTask;
    fn make(new: &NewTask, key: &TaskID) -> Task {
        Task {
            task_id: *key,
            task_name: new.task_name.clone(),
            project_id: new.project_id,
        }
    }
}

pub struct Man<T> where T: DBType {
    data: HashMap<T::Key,T>,
    next_k: T::Key,
}

impl<T> Man<T> where T: DBType 
{
    pub fn new(k: &T::Key) -> Self {
        return Self {
            data: HashMap::new(),
            next_k: *k,
        };
    }
    
    pub fn insert(&mut self, item: T::New) -> T::Key {
        let k = self.next_k;
        let v = T::make(&item, &k);
        self.data.insert(k, v);
        self.next_k.inc();
        return k;
    }

    pub fn get(&self, k: T::Key) -> Option<&T> {
        return self.data.get(&k);
    }

    pub fn list(&self) -> Vec<T> {
        return self.data.values().cloned().collect();
    }
}

pub struct Env {
    pub(super) projects: Man<Project>,
    pub(super) workers:  Man<Worker>,
    pub(super) tasks:    Man<Task>,
    pub(super) workertasks: HashSet<(TaskID,WorkerID)>,
}

impl Env {
    pub fn new() -> Env {
        return Env {
            projects: Man::new(&ProjectID(1)),
            workers:  Man::new(&WorkerID(1)),
            tasks:    Man::new(&TaskID(1)),
            workertasks: HashSet::new(),
        };
    }
}
