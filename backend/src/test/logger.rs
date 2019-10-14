use crate::traits::Logger;
use super::*;

impl Logger for Env {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
