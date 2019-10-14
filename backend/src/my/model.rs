use crate::traits::Model;
use crate::data::*;
use common::*;
use crate::schema::*;
use crate::*;
use super::*;
use diesel::prelude::*;

macro_rules! last_id (
    ($table: ident :: $field: ident, $db_con: expr) => {
        $table::table.select($table::$field)
            .order($table::$field.desc())
            .first($db_con)
            .unwrap()
    }
);

impl Model for Env {
    fn add_project(&mut self, item: NewProject) -> ProjectID {
        let db_con = db::open_db();
        diesel::insert_into(projects::table)
            .values(&item)
            .execute(&db_con)
            .unwrap();
        return last_id!(projects::project_id, &db_con);
    }

    fn add_worker(&mut self, item: NewWorker) -> WorkerID {
        let db_con = db::open_db();
        diesel::insert_into(workers::table)
            .values(&item)
            .execute(&db_con)
            .unwrap();
        return last_id!(workers::worker_id, &db_con);
    }
    
    fn add_task(&mut self, item: NewTask) -> TaskID {
        let db_con = db::open_db();
        diesel::insert_into(tasks::table)
            .values(&item)
            .execute(&db_con)
            .unwrap();
        return last_id!(tasks::task_id, &db_con);
    }

    fn get_projects(&self) -> Vec<Project> {
        let db_con = db::open_db();
        return projects::table.load::<Project>(&db_con)
            .unwrap();
    }
    
    fn get_workers(&self) -> Vec<Worker> {
        let db_con = db::open_db();
        return workers::table.load::<Worker>(&db_con)
            .unwrap();
    }
    
    fn get_tasks(&self) -> Vec<Task> {
        let db_con = db::open_db();
        return tasks::table.load::<Task>(&db_con)
            .unwrap();
    }
    
    fn get_project(&self, _: ProjectID) -> Option<Project> {
        unimplemented!()
    }

    fn get_worker(&self, _: WorkerID) -> Option<Worker> {
        unimplemented!()
    }

    fn get_task(&self, _: TaskID) -> Option<Task> {
        unimplemented!()
    }
    
    fn get_project_tasks(&self, project_id: ProjectID) -> Vec<Task> {
        let db_con = db::open_db();
        return tasks::table.filter(tasks::project_id.eq(project_id))
            .get_results::<Task>(&db_con)
            .unwrap();
    }

    fn get_worker_tasks(&self, worker_id: WorkerID) -> Vec<Task> {
        let db_con = db::open_db();
//        let wts = workertasks::table.load::<WorkerTask>(&db_con)
//            .unwrap();
//        for i in wts {
//            println!("{:?}", i);
//        }

        let mut rows = tasks::table
            .inner_join(workertasks::table.on(
                workertasks::task_id.eq(tasks::task_id)
                .and(workertasks::worker_id.eq(worker_id))
                ))
            .select((tasks::task_id, tasks::task_name, tasks::project_id))
            .load::<(TaskID,TaskName,ProjectID)>(&db_con)
            .unwrap();

        let row_to_task = |(task_id, task_name, project_id)| {
            Task{ task_id, task_name, project_id }
        };

        return rows.drain(..).map(row_to_task).collect();
    }
    
    fn assign_task(&mut self, item: NewWorkerTask) {
        let db_con = db::open_db();
        diesel::insert_into(workertasks::table)
            .values(&item)
            .execute(&db_con)
            .unwrap();
    }

    fn unassign_task(&mut self, item: NewWorkerTask) {
        let db_con = db::open_db();
        let is_same = workertasks::worker_id.eq(item.worker_id)
            .and(workertasks::task_id.eq(item.task_id));
        diesel::delete(workertasks::table)
            .filter(is_same)
            .execute(&db_con)
            .unwrap();
    }
}

//impl Model for Env {
//    fn add_record(&mut self, record: User) {
//        let db_con = db::open_db();
//        db::add_user(&db_con, record.last_name, record.first_name);
//        self.data.push(record);
//    }
//    
//    fn get_records(&self) -> Vec<User> {
//        return self.data.clone();
//    }
//}
