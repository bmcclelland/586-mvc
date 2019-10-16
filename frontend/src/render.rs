use common::*;
use yew::prelude::*;
use crate::model::*;
use crate::msg::*;

fn view_buttons(model: &Model) -> Html<Model> {
    html! {
        <div>
            <div>
                <input
                    placeholder="Project Name"
                    value=&model.inputs.project_name.0
                    oninput=|e| Msg::UpdateProjectInput(ProjectName(e.value))
                />
                <button onclick=|_| Msg::AddProject>{ "Add Project" }</button>
            </div>
            <div>
                <input
                    placeholder="Worker Name"
                    value=&model.inputs.worker_name.0
                    oninput=|e| Msg::UpdateWorkerInput(WorkerName(e.value))
                />
                <button onclick=|_| Msg::AddWorker>{ "Add Worker" }</button>
            </div>
            <button onclick=|_| Msg::GetProjects>{ "Get Projects" }</button>
            <button onclick=|_| Msg::GetWorkers>{ "Get Workers" }</button>
            <button onclick=|_| Msg::GetTasks>{ "Get Tasks" }</button>
        </div>
    }
}
        
fn view_view(model: &Model) -> Html<Model> {
    match &model.view {
        View::Null=> html! {
            <div>
                <p>{ "(No view)" }</p>
            </div>
        },
        View::Projects(projects) => html! {
            <div>
                <p>{"PROJECTS: "}</p>
                <ul>{ for projects.iter().map(view_project) }</ul>
            </div>
        },
        View::Workers(workers) => html! {
            <div>
                <p>{"WORKERS: "}</p>
                <ul>{ for workers.iter().map(view_worker) }</ul>
            </div>
        },
        View::Tasks(tasks) => html! {
            <div>
                <p>{"TASKS: "}</p>
                <ul>{ for tasks.iter().map(view_task) }</ul>
            </div>
        },
    }
}

#[allow(dead_code)]
fn view_debug(model: &Model) -> Html<Model> {
    html! {
        <div>
            <p>{ for model.debug.iter().cloned().map(view_stuff) }</p>
        </div>
    }
}

#[allow(dead_code)]
fn view_stuff(s: &'static str) -> Html<Model> {
    html! {
        <span>{ s }{" "}</span> 
    }
}

fn view_project(project: &Project) -> Html<Model> {
    let project_name = project.project_name.clone();
    let project_id = project.project_id;

    html! {
        <li>
            <span>{ project_id.0 }{": "}</span> 
            <span>{ project_name.0 }</span>
            <button onclick=|_| Msg::DeleteProject(project_id)>{ "Delete" }</button>
        </li>
    }
}

fn view_worker(worker: &Worker) -> Html<Model> {
    let worker_name = worker.worker_name.clone();
    let worker_id = worker.worker_id;

    html! {
        <li>
            <span>{ worker_id.0 }{": "}</span> 
            <span>{ worker_name.0 }</span>
            <button onclick=|_| Msg::DeleteWorker(worker_id)>{ "Delete" }</button>
        </li>
    }
}

fn view_task(task: &Task) -> Html<Model> {
    html! {
        <li>
            <span>{ task.task_id.0 }{": "}</span> 
            <span>{ task.task_name.0.clone() }{": "}</span>
            <span>{ task.project_id.0 }</span>
        </li>
    }
}
            
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                { view_buttons(&self) }
//                { view_debug(&self) }
                { view_view(&self) }
            </div>
        }
    }
}
