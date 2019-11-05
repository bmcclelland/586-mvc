use super::*;

impl Action for AddTaskAction {
    fn perms(&self) -> PermReq {
        perms!(CreateTask)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        let task_id = env.add_task(NewTask{
            task_name: self.0.task_name.clone(),
            project_id: self.0.project_id,
        });
        Box::new(task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Env;
    
    #[test]
    fn add_task() {
        let model = &mut Env::default();
        
        let project_id = model.add_project(NewProject {
            project_name: ProjectName("test".into()),
        });
        
        let count_before = model.get_tasks().len();
        let task_name = TaskName("test".into());
        let action = AddTaskAction(AddTaskParams {
            task_name: task_name.clone(),
            project_id,
        });

        let task_id = round_trip(action.execute(model));
        let task_real = model.get_task(task_id);
        let count_after = model.get_tasks().len();
        let task_ideal = Task { task_id, project_id, task_name };

        assert_eq!(count_after, count_before + 1);
        assert!(task_real.is_some());
        assert!(task_real.unwrap() == task_ideal);
    }
}
