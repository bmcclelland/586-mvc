use common::*;
use yew::prelude::*;
use yew::services::fetch::*;
use yew::services::storage::*;

pub enum View {
    Null,
    Projects(Vec<Project>),
    Workers(Vec<Worker>),
    Tasks(Vec<Task>),
}

pub struct Inputs {
    pub project_name: ProjectName,
    pub worker_name:  WorkerName,
    pub task_name:    TaskName,
}

pub struct Model {
    pub fetcher: FetchService,
    pub storage: StorageService,
    pub link: ComponentLink<Model>,
    pub debug: Vec<&'static str>,
    pub task: Option<FetchTask>,
    pub view: View,
    pub inputs: Inputs,
}

impl Model {
    pub fn new(link: ComponentLink<Self>) -> Self {
        Self {
            fetcher: FetchService::new(),
            storage: StorageService::new(Area::Local),
            link,
            debug: vec![],
            task: None,
            view: View::Null,
            inputs: Inputs {
                project_name: ProjectName("".into()),
                worker_name:  WorkerName("".into()),
                task_name:    TaskName("".into()),
            },
        }
    }
}
