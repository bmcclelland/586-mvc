use common::*;
use yew::prelude::*;
use crate::model::*;
use crate::msg::*;

fn view_buttons(model: &Model) -> Html<Model> {
    html! {
        <div>
            <input
                placeholder="Project Name"
                value=&model.inputs.project_name.0
                oninput=|e| Msg::UpdateProjectInput(ProjectName(e.value))
            />
            <button onclick=|_| Msg::AddProject>{ "Add Project" }</button>
            <button onclick=|_| Msg::GetProjects>{ "Get Projects" }</button>
            <button onclick=|_| Msg::GetWorkers>{ "Get Workers" }</button>
            <button onclick=|_| Msg::GetTasks>{ "Get Tasks" }</button>
        </div>
    }
}
        
fn view_view(model: &Model) -> Html<Model> {
    match &model.view {
        NullView => html! {
            <div>
                <p>{ "NULL" }</p>
            </div>
        },
        ProjectsView(projects) => html! {
            <div>
                <p>{"PROJECTS: "}</p>
                <ul>{ for projects.iter().map(view_project) }</ul>
            </div>
        },
        WorkersView(workers) => html! {
            <div>
                <p>{"WORKERS: "}</p>
                <ul>{ for workers.iter().map(view_worker) }</ul>
            </div>
        },
        TasksView(tasks) => html! {
            <div>
                <p>{"TASKS: "}</p>
                <ul>{ for tasks.iter().map(view_task) }</ul>
            </div>
        },
    }
}

//fn view_debug(model: &Model) -> Html<Model> {
//    html! {
//        <div>
//            <p>{ for model.stuff.iter().cloned().map(view_stuff) }</p>
//        </div>
//    }
//}
//
//fn view_stuff(s: &'static str) -> Html<Model> {
//    html! {
//        <span>{ s }{" "}</span> 
//    }
//}

fn view_project(project: &Project) -> Html<Model> {
    html! {
        <li>
            <span>{ project.project_id.0 }{": "}</span> 
            <span>{ project.project_name.0.clone() }</span>
        </li>
    }
}


fn view_worker(worker: &Worker) -> Html<Model> {
    html! {
        <li>
            <span>{ worker.worker_id.0 }{": "}</span> 
            <span>{ worker.worker_name.0.clone() }</span>
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
