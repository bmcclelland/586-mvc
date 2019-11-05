use super::*;
    
impl Action for AddProjectAction {
    fn perms(&self) -> PermReq {
        perms!(CreateProject)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        let project_id = model.add_project(NewProject{
            project_name: self.0.project_name.clone()
        })?;
        Ok(Box::new(project_id))
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::test::Env;
//
//    fn mk_action(project_name: &str) -> AddProjectAction {
//        let project_name = ProjectName(project_name.into());
//        AddProjectAction(AddProjectParams {
//            project_name: project_name.clone(),
//        })
//    }
//
//    #[test]
//    fn add_project() {
//        let model = &mut Env::default();
//        
//        let count_before = model.get_projects().len();
//        let project_id = round_trip(action.execute(model));
//        let project_real = model.get_project(project_id);
//        let count_after = model.get_projects().len();
//        let project_ideal = Project { 
//            project_id, 
//            project_name: action.0.project_name.clone(),
//        };
//
//        assert_eq!(count_after, count_before + 1);
//        assert!(project_real.is_some());
//        assert!(project_real.unwrap() == project_ideal);
//    }
//      
//    #[test]
//    fn add_bad_project() {
//        let model = &mut Env::new();
//        let action = mk_action("");
//        
//        let count_before = model.get_projects().len();
//        let project_id = round_trip(action.execute(model));
//        let count_after = model.get_projects().len();
//
//        assert!(project_id.is_none());
//        assert_eq!(count_after, count_before);
//    }
//}
