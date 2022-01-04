use std::time::Duration;
use crate::single_thread_executor::Executor;

mod single_thread_executor;

fn main() {
    let w = Executor::new();

    std::thread::sleep(Duration::new(3, 0));
    println!("Main");
}

