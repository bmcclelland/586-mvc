use erased_serde::Serialize;
//use super::Model;
use crate::prod::Model;
use common::*;
use crate::perms::PermReq;

pub trait Action {
    fn execute(&self, _: &mut Model) -> AppResult<Box<dyn Serialize>>;
    fn perms(&self) -> PermReq;
}
