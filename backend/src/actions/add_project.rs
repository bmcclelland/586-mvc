use super::*;
    
impl Action for AddProjectAction {
    fn perms(&self) -> PermReq {
        perms!(CreateProject)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        let project_id = env.add_project(NewProject{
            project_name: self.0.project_name.clone()
        });
        Box::new(project_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::Env;

    #[test]
    fn add_project() {
        let model = &mut Env::default();
        
        let project_name = ProjectName("test_project".into());
        let action = AddProjectAction(AddProjectParams {
            project_name: project_name.clone(),
        });
        let count_before = model.get_projects().len();
        let project_id = round_trip(action.execute(model));
        let project_real = model.get_project(project_id);
        let count_after = model.get_projects().len();
        let project_ideal = Project { project_id, project_name };

        assert_eq!(count_after, count_before + 1);
        assert!(project_real.is_some());
        assert!(project_real.unwrap() == project_ideal);
    }
}
