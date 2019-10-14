use super::*;

impl Action for GetWorkersAction {
    fn perms(&self) -> PermReq {
        perms!(ReadWorker)
    }

    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize> {
        return Box::new(env.get_workers());
    }
}
