use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use crate::single_thread_executor::Executor;

mod single_thread_executor;

fn main() {
    let mut executor = Executor::new();
    let mut f = || -> () {println!("Hello!1")};
    executor.append(Box::new(f));
    sleep(Duration::from_secs(1));
    let mut f = || -> () {println!("Hello!2")};
    sleep(Duration::from_secs(1));
}

