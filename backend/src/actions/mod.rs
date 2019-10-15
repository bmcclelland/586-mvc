#[macro_use] mod macros;
use crate::traits::Action;
type ActionFn = fn(&Request) -> Option<Box<dyn Action>>;

// Listing "some_name" will create a struct SomeNameAction(SomeNameParams)
//  and wire it to the server router at /api/some_name.
// Implement Action for SomeNameAction in actions/some_name.rs.
// SomeNameParams should be defined in the common crate.
routable_actions!(
    add_project,
    add_worker,
    add_task,
    get_workers,
    get_projects,
    get_tasks,
    delete_project,
    delete_worker,
);

/////////////////////////////////////
// Convenience imports for submodules, use super::*.

use erased_serde::Serialize;
use serde::Deserialize;
use crate::data::*;
use crate::traits::*;
use crate::perms::*;
use common::*;

use rouille::{
    Request,
    input::json::*,
};

fn try_json<T>(request: &Request) -> Option<T>
    where for<'de> T: Deserialize<'de>
{
    let a = json_input(request);
    match a {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}

// Gets a T out of a dyn Serialize, if you're right about the T.
// This is needed to test Action::execute.
#[cfg(test)]
fn round_trip<T>(t: Box<dyn Serialize>) -> T
    where T: for<'de> Deserialize<'de>
{
    let json = serde_json::to_string(&*t).unwrap();
    serde_json::from_str(&json).unwrap()
}

