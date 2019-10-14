use backend::*;

fn main() {
    let env = my::Env::default();
    server::serve(env);
}
