use super::*;

impl Action for AddWorkerAction {
    fn perms(&self) -> PermReq {
        perms!(CreateWorker)
    }

    fn execute(&self, model: &mut Model) -> AppResult<Box<dyn Serialize>> {
        let worker_id = model.add_worker(Worker{
            id: WorkerID::default(),
            name: self.0.name.clone(),
        })?;
        Ok(Box::new(worker_id))
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::test::Env;
//
//    #[test]
//    fn add_worker() {
//        let model = &mut Env::default();
//        
//        let count_before = model.get_workers().len();
//        let worker_name = WorkerName("test".into());
//        let action = AddWorkerAction(AddWorkerParams {
//            worker_name: worker_name.clone(),
//        });
//        let worker_id = round_trip(action.execute(model));
//        let worker_real = model.get_worker(worker_id);
//        let count_after = model.get_workers().len();
//        let worker_ideal = Worker { worker_id, worker_name };
//
//        assert_eq!(count_after, count_before + 1);
//        assert!(worker_real.is_some());
//        assert!(worker_real.unwrap() == worker_ideal);
//    }
//}
