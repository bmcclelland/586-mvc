use super::*;

impl Action for GetProjectsAction {
    fn perms(&self) -> PermReq {
        perms!(ReadProject)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        Box::new(env.get_projects())
    }
}
