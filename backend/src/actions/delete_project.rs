use super::*;
    
impl Action for DeleteProjectAction {
    fn perms(&self) -> PermReq {
        perms!(DeleteProject)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        let result: bool = env.delete_project(self.0.project_id);
        Box::new(result)
    }
}
