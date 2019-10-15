use super::*;
    
impl Action for DeleteWorkerAction {
    fn perms(&self) -> PermReq {
        perms!(DeleteWorker)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        let result: bool = env.delete_worker(self.0.worker_id);
        return Box::new(result);
    }
}
