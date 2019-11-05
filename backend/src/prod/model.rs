use common::*;
use common::schema::*;
use crate::db::Insert;
use diesel::prelude::*;

pub struct Model {
    db: SqliteConnection,
}

pub struct WorkerTaskCount {
    worker_id: WorkerID,
    task_count: usize,
}

impl Model {
    pub fn new(db: SqliteConnection) -> Self {
        Self {
            db
        }
    }

    pub fn add_project(&mut self, project: Project) -> AppResult<ProjectID> {
        project.insert(&self.db)
    }

    pub fn add_worker(&mut self, worker: Worker) -> AppResult<WorkerID> {
        worker.insert(&self.db)
    }
    
    pub fn add_task(&mut self, task: Task) -> AppResult<TaskID> {
        task.insert(&self.db)
    }
    
    pub fn delete_project(&self, project_id: ProjectID) -> AppResult<()> {
        diesel::delete(projects::table)
            .filter(projects::id.eq(project_id))
            .execute(&self.db)?;
        Ok(())
    }
    
    pub fn delete_worker(&self, worker_id: WorkerID) -> AppResult<()> {
        diesel::delete(workers::table)
            .filter(workers::id.eq(worker_id))
            .execute(&self.db)?;
        Ok(())
    }

    pub fn get_projects(&self) -> AppResult<Vec<Project>> {
        let result = projects::table.load(&self.db)?;
        Ok(result)
    }
    
    pub fn get_workers(&self) -> AppResult<Vec<Worker>> {
        let result = workers::table.load(&self.db)?;
        Ok(result)
    }
    
    pub fn get_tasks(&self) -> AppResult<Vec<Task>> {
        let result = tasks::table.load(&self.db)?;
        Ok(result)
    }
    
    pub fn get_project(&self, _: ProjectID) -> AppResult<Project> {
        unimplemented!()
    }

    pub fn get_worker(&self, _: WorkerID) -> AppResult<Worker> {
        unimplemented!()
    }

    pub fn get_task(&self, _: TaskID) -> AppResult<Task> {
        unimplemented!()
    }
    
    pub fn get_project_tasks(&self, project_id: ProjectID) -> AppResult<Vec<Task>> {
        let project: Project = projects::table.find(project_id).first(&self.db)?;
        let result = Task::belonging_to(&project).load(&self.db)?;
        Ok(result)
    }

    pub fn get_worker_tasks(&self, worker_id: WorkerID) -> AppResult<Vec<Task>> {
        let worker: Worker = workers::table.find(worker_id).first(&self.db)?;
        let result = Task::belonging_to(&worker).load(&self.db)?;
        Ok(result)
    }
    
    pub fn assign_task(&mut self, task_id: TaskID, worker_id: WorkerID) -> AppResult<()> {
        let mut task: Task = tasks::table.find(task_id).first(&self.db)?;
        task.worker_id = Some(worker_id);
        task.save_changes::<Task>(&self.db)?;            
        Ok(())
    }

    pub fn unassign_task(&mut self, task_id: TaskID) -> AppResult<()> {
        let mut task: Task = tasks::table.find(task_id).first(&self.db)?;
        task.worker_id = None; 
        task.save_changes::<Task>(&self.db)?;            
        Ok(())
    }

    pub fn worker_taskcount(&mut self)
        -> AppResult<Vec<WorkerTaskCount>>
    {
        workers::table
            .load(&self.db)?
            .into_iter()
            .map(|w: Worker| -> AppResult<WorkerTaskCount> {
                let tasks: Vec<Task> = 
                    Task::belonging_to(&w).load(&self.db)?;
                Ok(WorkerTaskCount {
                    worker_id: w.id,
                    task_count: tasks.len(),
                })
            })
            .collect()
    }
}
