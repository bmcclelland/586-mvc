use backend::*;

fn main() {
    let env = prod::Env::default();
    server::serve(env);
}
