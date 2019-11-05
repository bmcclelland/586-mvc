use super::*;

impl Action for GetProjectsAction {
    fn perms(&self) -> PermReq {
        perms!(ReadProject)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        Ok(Box::new(model.get_projects()?))
    }
}
