use common::*;
use yew::prelude::*;
use yew::services::fetch::*;

pub enum View {
    NullView,
    ProjectsView(Vec<Project>),
    WorkersView(Vec<Worker>),
    TasksView(Vec<Task>),
}

pub use View::*;

pub struct Inputs {
    pub project_name: ProjectName,
}

pub struct Model {
    pub fetcher: FetchService,
    pub link: ComponentLink<Model>,
    pub stuff: Vec<&'static str>,
    pub task: Option<FetchTask>,
    pub view: View,
    pub inputs: Inputs,
}

impl Model {
    pub fn new(_: (), link: ComponentLink<Self>) -> Self {
        Self {
            fetcher: FetchService::new(),
            link: link,
            stuff: vec![],
            task: None,
            view: NullView,
            inputs: Inputs {
                project_name: ProjectName("".into()),
            },
        }
    }
}
