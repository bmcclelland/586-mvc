macro_rules! routable_actions (
    ($($mod: ident),+$(,)?) => {
        // Given $mod = something:
        
        // Declare each SomethingAction(SomethingParams).
        $(procs::action_types!($mod);)+

        // Declare each routers::something() which just pulls the
        //  JSON out of the request and constructs SomethingAction.
        mod routers {
            use super::*;
            $(procs::action_func!($mod);)+
        }
        
        // Declare each module something.
        $(mod $mod;)+

        // Create a function which, given "something", will return
        //  the function routers::something.
        pub fn lookup_action_fn(action: &str) -> Option<ActionFn>
        {
            match action {
                $(stringify!($mod) => { Some(routers::$mod) },)+
                _ => None,
            }
        }
    }
);
