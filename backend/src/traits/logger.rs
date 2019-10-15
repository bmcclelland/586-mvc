pub trait Logger: Send+Sync {
    fn log(&self, msg: &str);
}
