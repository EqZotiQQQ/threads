use std::sync::Arc;
use std::time::Duration;
use crate::single_thread_executor::Executor;

mod single_thread_executor;

fn main() {
    let mut executor = Executor::new();
    let mut f = || -> () {println!("Hello!")};
    executor.append(Box::new(f));
}

