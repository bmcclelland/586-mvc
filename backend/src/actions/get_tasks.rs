use super::*;

impl Action for GetTasksAction {
    fn perms(&self) -> PermReq {
        perms!(ReadTask)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        Box::new(env.get_tasks())
    }
}
