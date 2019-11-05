use super::*;
    
impl Action for DeleteWorkerAction {
    fn perms(&self) -> PermReq {
        perms!(DeleteWorker)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        let result = model.delete_worker(self.0.worker_id)?;
        Ok(Box::new(result))
    }
}
