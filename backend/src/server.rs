use crate::traits::*;
use crate::actions::lookup_action_fn;
use crate::prod::Model;
use rouille::{ router, Response, Request, start_server };
use std::sync::RwLock;
use crate::db;

pub fn serve(env: impl 'static+Logger) -> !
{
    let addr = "0.0.0.0:8001"; // TODO hardcoded
    start_server(addr, request_handler(env)); 
}
             
// TODO
fn validate_action(
    _env: &(impl Logger),
    _action: &dyn Action,
    )
    -> Option<()>
{
    Some(())
}

fn handle_action(
    env: &mut (impl Logger),
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
        env.log("404: action_fn failed");
        return Response::empty_404();
    });
   
    try_or!(validate_action(env, &*action), {
        env.log("400: Failed to validate");
        return Response::empty_400();
    });

    let mut model = Model::new(db::open());
        
    match action.execute(&mut model) {
        Ok(response_data) => {
            env.log("200: Responding");
            Response::json(&response_data)
                .with_unique_header("Access-Control-Allow-Origin", "*")
        }
        Err(err) => {
            env.log("200: Responding with error");
            Response::json(&format!("Error: {}", err))
                .with_unique_header("Access-Control-Allow-Origin", "*")

        }
    }
}

fn request_handler(env: impl Logger)
    -> impl Fn(&Request) -> Response
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
                env_guard.log("404: No route");
                Response::empty_404()
            }
        )
    }
}

