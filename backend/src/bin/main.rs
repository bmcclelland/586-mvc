use backend::*;

fn main() {
    let env = prod::Env::new();
    server::serve(env);
}
