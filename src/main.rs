use std::thread::sleep;
use std::time::Duration;
use crate::single_thread_executor::Executor;

mod single_thread_executor;

fn main() {
    let mut executor = Executor::new();
    println!("Hello from main 1");
    executor.start();
    println!("Hello from main 2");
    let mut f = || -> () {println!("Hello from closure 1")};


    sleep(Duration::from_secs(1));

    executor.append(Box::new(f)).unwrap();
    println!("Hello from main 3");
    let mut f = || -> () {println!("Hello from closure 2")};
    executor.append(Box::new(f)).unwrap();
    println!("Hello from main 4");
    // sleep(Duration::from_secs(2));
}

