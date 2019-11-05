//use crate::traits::Model;
use crate::data::*;
use common::*;
use crate::schema::*;
use crate::*;
//use super::*;
use diesel::prelude::*;

macro_rules! last_id (
    ($table: ident :: $field: ident, $db_con: expr) => {
        $table::table.select($table::$field)
            .order($table::$field.desc())
            .first($db_con)
    }
);

pub struct Model {
    db: SqliteConnection,
}

impl Model {
    pub fn new(db: SqliteConnection) -> Self {
        Self {
            db
        }
    }

    pub fn add_project(&mut self, item: NewProject) -> AppResult<ProjectID> {
        diesel::insert_into(projects::table)
            .values(&item)
            .execute(&self.db)?;
        let result = last_id!(tasks::task_id, &self.db)?;
        Ok(result)
    }

    pub fn add_worker(&mut self, item: NewWorker) -> AppResult<WorkerID> {
        diesel::insert_into(workers::table)
            .values(&item)
            .execute(&self.db)?;
        let result = last_id!(tasks::task_id, &self.db)?;
        Ok(result)
    }
    
    pub fn add_task(&mut self, item: NewTask) -> AppResult<TaskID> {
        diesel::insert_into(tasks::table)
            .values(&item)
            .execute(&self.db)?;
        let result = last_id!(tasks::task_id, &self.db)?;
        Ok(result)
    }
    
    pub fn delete_project(&self, project_id: ProjectID) -> AppResult<()> {
        let _deleted_rows = diesel::delete(projects::table)
            .filter(projects::project_id.eq(project_id))
            .execute(&self.db)?;
        Ok(()) //deleted_rows >= 1)
    }
    
    pub fn delete_worker(&self, worker_id: WorkerID) -> AppResult<()> {
        let _deleted_rows = diesel::delete(workers::table)
            .filter(workers::worker_id.eq(worker_id))
            .execute(&self.db)?;
        Ok(()) //deleted_rows >= 1)
    }

    pub fn get_projects(&self) -> AppResult<Vec<Project>> {
        let result = projects::table.load::<Project>(&self.db)?;
        Ok(result)
    }
    
    pub fn get_workers(&self) -> AppResult<Vec<Worker>> {
        let result = workers::table.load::<Worker>(&self.db)?;
        Ok(result)
    }
    
    pub fn get_tasks(&self) -> AppResult<Vec<Task>> {
        let result = tasks::table.load::<Task>(&self.db)?;
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
        let result = tasks::table.filter(tasks::project_id.eq(project_id))
            .get_results::<Task>(&self.db)?;
        Ok(result)
    }

    pub fn get_worker_tasks(&self, worker_id: WorkerID) -> AppResult<Vec<Task>> {
        let mut rows = tasks::table
            .inner_join(workertasks::table.on(
                workertasks::task_id.eq(tasks::task_id)
                .and(workertasks::worker_id.eq(worker_id))
                ))
            .select((tasks::task_id, tasks::task_name, tasks::project_id))
            .load::<(TaskID,TaskName,ProjectID)>(&self.db)?;

        let row_to_task = |(task_id, task_name, project_id)| {
            Task{ task_id, task_name, project_id }
        };

        Ok(rows.drain(..).map(row_to_task).collect())
    }
    
    pub fn assign_task(&mut self, item: NewWorkerTask) -> AppResult<()> {
        diesel::insert_into(workertasks::table)
            .values(&item)
            .execute(&self.db)?;
        Ok(())
    }

    pub fn unassign_task(&mut self, item: NewWorkerTask) -> AppResult<()> {
        let is_same = workertasks::worker_id.eq(item.worker_id)
            .and(workertasks::task_id.eq(item.task_id));
        diesel::delete(workertasks::table)
            .filter(is_same)
            .execute(&self.db)?;
        Ok(())
    }
}
