use crate::traits::*;
use crate::actions::lookup_action_fn;
use rouille::{ router, Response, Request, start_server };
use std::sync::RwLock;
use erased_serde::Serialize;

pub fn serve<EnvT>(env: EnvT) -> !
    where EnvT: 'static+Logger+Model
{
    let addr = "0.0.0.0:8001"; // TODO hardcoded
    start_server(addr, request_handler(env)); 
}
             
// TODO
fn validate_action(
    _env: &(impl Model+Logger),
    _action: &dyn Action,
    )
    -> Option<()>
{
    Some(())
}

fn handle_action(
    env: &mut (impl Model+Logger),
    action_name: &str, 
    request: &Request
    )
    -> Response
{
    env.log(&format!("POST: {}", request.raw_url()));
    
    let action_fn = try_or!(lookup_action_fn(action_name), {
        env.log(&format!("404: no action {} found", action_name));
        return Response::empty_404();
    });

    let action = try_or!(action_fn(request), {
        env.log(&format!("404: action_fn failed"));
        return Response::empty_404();
    });
   
    let _ = try_or!(validate_action(env, &*action), {
        env.log(&format!("400: Failed to validate"));
        return Response::empty_400();
    });
        
    let response_data : Box<dyn Serialize> = action.execute(env);
    env.log(&format!("200: Responding"));
    return Response::json(&response_data)
        .with_unique_header("Access-Control-Allow-Origin", "*")
}

fn request_handler<EnvT>(
    env: EnvT
    ) 
    -> impl Fn(&Request) -> Response
    where EnvT: Model+Logger
{
    let env = RwLock::new(env);

    move |request| {

        router!(request,
            (POST) (/api/{action_name : String}) => { 
                let mut env_guard = env.write().unwrap();
                handle_action(&mut *env_guard, &action_name, &request)
            },
            
            (OPTIONS) (/api/{_action_name: String}) => {
                Response::text("")
                    .with_unique_header("Access-Control-Allow-Origin", "*")
                    .with_unique_header("Access-Control-Allow-Headers", "*")
            },
            
            _ => {
                let env_guard = env.write().unwrap();
                env_guard.log(
                    &format!("{}: {}", request.method(), request.raw_url())
                );
                env_guard.log(&format!("404: No route"));
                Response::empty_404()
            }
        )
    }
}

