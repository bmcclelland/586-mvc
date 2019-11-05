use super::*;

impl Action for GetTasksAction {
    fn perms(&self) -> PermReq {
        perms!(ReadTask)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        Ok(Box::new(model.get_tasks()?))
    }
}
