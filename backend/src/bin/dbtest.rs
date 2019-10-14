use backend::*;
use backend::data::*;
use backend::traits::*;
use common::*;

fn wn(name: &str) -> WorkerName {
    WorkerName(name.to_string())
}

fn tn(name: &str) -> TaskName {
    TaskName(name.to_string())
}

fn pn(name: &str) -> ProjectName {
    ProjectName(name.to_string())
}

fn main() {
    let mut env = my::Env::default();
    
    env.add_worker(NewWorker{ worker_name: wn("man"), }); 
    env.add_worker(NewWorker{ worker_name: wn("otherman"), });
    
    let pid = env.add_project(NewProject{
        project_name: pn("big project"),
    });
    
    env.add_task(NewTask{ task_name: tn("do thing"), project_id: pid });
    env.add_task(NewTask{ task_name: tn("review thing"), project_id: pid });
    env.add_task(NewTask{ task_name: tn("do anotherthing"), project_id: pid });
    env.add_task(NewTask{ task_name: tn("kill everyone"), project_id: pid });
    
    println!("Projects:");
    for i in env.get_projects() { println!("{:?}", i); }
    
    println!("Workers:");
    for i in env.get_workers() { println!("{:?}", i); }
    
    println!("Tasks:");
    for i in env.get_tasks() { println!("{:?}", i); }
    
    println!("Project Tasks:");
    for i in env.get_project_tasks(pid) { println!("{:?}", i); }

    for i in 1..=4 {
        env.assign_task(NewWorkerTask {
            worker_id: WorkerID(1), 
            task_id:   TaskID(i),
        });
    }
    
    for i in 1..=4 {
        env.assign_task(NewWorkerTask {
            worker_id: WorkerID(2), 
            task_id:   TaskID(i),
        });
    }

    println!("Worker Tasks:");
    for i in env.get_worker_tasks(WorkerID(1)) { println!("{:?}", i); }
}
