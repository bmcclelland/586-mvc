use super::*;
    
impl Action for DeleteProjectAction {
    fn perms(&self) -> PermReq {
        perms!(DeleteProject)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        let result = model.delete_project(self.0.project_id)?;
        Ok(Box::new(result))
    }
}
