use erased_serde::Serialize;
use super::Model;
use crate::perms::PermReq;

pub trait Action {
    fn execute(&self, env: &mut dyn Model) -> Box<dyn Serialize>;
    fn perms(&self) -> PermReq;
}
