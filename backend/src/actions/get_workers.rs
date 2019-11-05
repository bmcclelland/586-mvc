use super::*;

impl Action for GetWorkersAction {
    fn perms(&self) -> PermReq {
        perms!(ReadWorker)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        Ok(Box::new(model.get_workers()?))
    }
}
