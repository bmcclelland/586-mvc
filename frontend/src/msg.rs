use common::*;
use yew::prelude::*;
use yew::services::fetch::*;
use yew::format::Json;
use failure::Error;
use serde::{Serialize};
use crate::model::*;

pub enum Msg {
    Noop,
    GetProjects,
    GetWorkers,
    GetTasks,
    AddProject,
    ViewWorkers(Vec<Worker>),
    ViewTasks(Vec<Task>),
    FetchFailed(i32),
    UpdateProjectInput(ProjectName),
    ViewProjects(Vec<Project>),
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
        Msg::Noop => {}
        Msg::FetchFailed(i) => {
            model.debug.push(match i {
                1 => "FetchFailed1",
                2 => "FetchFailed2",
                _ => "FetchFailed?",
            });
            model.task = None;
        }
        Msg::UpdateProjectInput(s) => {
            model.debug.push("UpdateProjectInput");
            model.inputs.project_name = s;
        }
        Msg::AddProject => {
            model.debug.push("AddProject");
            let payload = AddProjectPayload {
                project_name: model.inputs.project_name.clone(),
            };
            let req = json_request(&payload, "add_project");
            fetch!(model, req, |_project_id: ProjectID| {
                Msg::GetProjects
            });
        },
        Msg::ViewProjects(projects) => {
            model.debug.push("ViewProjects");
            model.view = ProjectsView(projects);
            model.task = None;
        }
        Msg::ViewWorkers(body) => {
            model.debug.push("ViewWorkers");
            model.view = WorkersView(body);
            model.task = None;
        }
        Msg::ViewTasks(body) => {
            model.debug.push("ViewTasks");
            model.view = TasksView(body);
            model.task = None;
        }
        Msg::GetProjects => {
            model.debug.push("GetProjects");
            let payload = GetProjectsPayload;
            let req = json_request(&payload, "get_projects");
            fetch!(model, req, |projects: Vec<Project>| {
                Msg::ViewProjects(projects)
            });
        }
        Msg::GetWorkers => {
            model.debug.push("GetWorkers");
            let payload = GetWorkersPayload;
            let req = json_request(&payload, "get_workers");
            fetch!(model, req, |workers: Vec<Worker>| {
                Msg::ViewWorkers(workers)
            });
        }
        Msg::GetTasks => {
            model.debug.push("GetTasks");
            let payload = GetTasksPayload;
            let req = json_request(&payload, "get_tasks");
            fetch!(model, req, |tasks: Vec<Task>| {
                Msg::ViewTasks(tasks)
            });
        }
    }
    true
}
