use common::*;
use yew::prelude::*;
use yew::services::fetch::*;
use yew::format::Json;
use failure::Error;
use serde::{Serialize};
use crate::model::*;

pub enum Msg {
    Noop,
    FetchFailed(i32),
    GetProjects,
    GetWorkers,
    GetTasks,
    AddProject,
    AddWorker,
    AddTask,
    DeleteProject(ProjectID),
    DeleteWorker(WorkerID),
    ViewProjects(Vec<Project>),
    ViewWorkers(Vec<Worker>),
    ViewTasks(Vec<Task>),
    UpdateProjectInput(ProjectName),
    UpdateWorkerInput(WorkerName),
    UpdateTaskInput(TaskName,ProjectID),
}
  
macro_rules! fetch(
    ($self: ident, $req: ident, $do: expr) => {
        let callback = $self.link.send_back(
            |rsp: Response<Json<Result<_,Error>>>| {
                let (meta, Json(body)) = rsp.into_parts();
                if meta.status.is_success() {
                    match body {
                        Ok(s)  => $do(s),
                        Err(_) => { 
                            Msg::FetchFailed(1)
                        }
                    }
                }
                else {
                    Msg::FetchFailed(2)
                }
            }
        );

        $self.task = Some($self.fetcher.fetch(
            $req,
            callback,
        ));
    }
);

fn json_request<'a, T>(body: &'a T, action: &str) 
    -> Request<Json<&'a T>> 
    where T: Serialize
{
    Request::post(format!("http://localhost:8001/api/{}", action)) // TODO
        .header("Content-Type", "application/json")
        .body(Json(body))
        .expect("Failed to build request")
}

pub fn update(model: &mut Model, msg: Msg) -> ShouldRender {
    match msg {
        Msg::Noop => {
            // Do absolutely nothing.
        }
        Msg::FetchFailed(i) => {
            model.debug.push(match i {
                1 => "FetchFailed1",
                2 => "FetchFailed2",
                _ => "FetchFailed?",
            });
            model.task = None;
        }
        Msg::AddProject => {
            model.debug.push("AddProject");
            let params = AddProjectParams {
                name: model.inputs.project_name.clone(),
            };
            let req = json_request(&params, "add_project");
            fetch!(model, req, |_project_id: ProjectID| {
                Msg::GetProjects
            });
            model.inputs.project_name.0.clear();
        }
        Msg::AddWorker => {
            model.debug.push("AddWorker");
            let params = AddWorkerParams {
                name: model.inputs.worker_name.clone(),
            };
            let req = json_request(&params, "add_worker");
            fetch!(model, req, |_worker_id: WorkerID| {
                Msg::GetWorkers
            });
            model.inputs.worker_name.0.clear();
        }
        Msg::DeleteProject(project_id) => {
            model.debug.push("DeleteProject");
            let params = DeleteProjectParams { project_id };
            let req = json_request(&params, "delete_project");
            fetch!(model, req, |_success: bool| {
                Msg::GetProjects
            });
        }
        Msg::DeleteWorker(worker_id) => {
            model.debug.push("DeleteWorker");
            let params = DeleteWorkerParams { worker_id };
            let req = json_request(&params, "delete_worker");
            fetch!(model, req, |_success: bool| {
                Msg::GetWorkers
            });
        }
        Msg::AddTask => {
            unimplemented!()
        }
        Msg::UpdateProjectInput(s) => {
            model.debug.push("UpdateProjectInput");
            model.inputs.project_name = s;
        }
        Msg::UpdateWorkerInput(s) => {
            model.debug.push("UpdateWorkerInput");
            model.inputs.worker_name = s;
        }
        Msg::UpdateTaskInput(_,_) => {
            unimplemented!();
        }
        Msg::ViewProjects(projects) => {
            model.debug.push("ViewProjects");
            model.view = View::Projects(projects);
            model.task = None;
        }
        Msg::ViewWorkers(body) => {
            model.debug.push("ViewWorkers");
            model.view = View::Workers(body);
            model.task = None;
        }
        Msg::ViewTasks(body) => {
            model.debug.push("ViewTasks");
            model.view = View::Tasks(body);
            model.task = None;
        }
        Msg::GetProjects => {
            model.debug.push("GetProjects");
            let params = GetProjectsParams;
            let req = json_request(&params, "get_projects");
            fetch!(model, req, |projects: Vec<Project>| {
                Msg::ViewProjects(projects)
            });
        }
        Msg::GetWorkers => {
            model.debug.push("GetWorkers");
            let params = GetWorkersParams;
            let req = json_request(&params, "get_workers");
            fetch!(model, req, |workers: Vec<Worker>| {
                Msg::ViewWorkers(workers)
            });
        }
        Msg::GetTasks => {
            model.debug.push("GetTasks");
            let params = GetTasksParams;
            let req = json_request(&params, "get_tasks");
            fetch!(model, req, |tasks: Vec<Task>| {
                Msg::ViewTasks(tasks)
            });
        }
    }
    true
}
