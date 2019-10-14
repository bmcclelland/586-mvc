pub trait Logger: Send+Sync {
    fn log(&self, msg: &str); // TODO make it take a &str
}
