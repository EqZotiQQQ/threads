use std::collections::VecDeque;
// use std::fmt::{Display, Formatter};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    NEW,
    RUNNING,
    STOPPED,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomError {
    NotStarted,
    STOPPED,
    AlreadyRunning,
}

pub struct Executor {
    executor: Option<thread::JoinHandle<()>>,
    state: State,
    cv: Arc<Condvar>,
    queue: Arc<Mutex<VecDeque<Box<dyn Fn() + Send>>>>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            executor: None,
            state: State::NEW,
            cv: Arc::new(Condvar::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn start(&mut self) -> Result<(), CustomError>{
        match self.state {
            State::NEW => {
                let queue = Arc::clone(&self.queue);
                let cv = Arc::clone(&self.cv);
                self.executor = Some(thread::spawn(move || {
                    loop {
                        println!("Queue1!");
                        let mut lock = queue.lock().unwrap();
                        println!("Queue2!");
                        if queue.lock().unwrap().is_empty() {
                            println!("Empty!");
                            cv.wait(lock).unwrap();
                        } else {
                            println!("Not empty!");
                        }
                    }
                }));
                self.state = State::RUNNING;
            }
            State::RUNNING => {return Err(CustomError::AlreadyRunning)}
            State::STOPPED => {return Err(CustomError::STOPPED)}
        }
        Ok(())
    }

    // Dumb shit is to use dyn Fn() + Send. It's literally illogical syntax.
    // Box<dyn Fn()> sugar for Box<dyn Fn() + 'static>
    pub fn append(&mut self, f: Box<dyn Fn() + Send + 'static>) -> Result<(), CustomError> {
        match self.state {
            State::NEW => {
                // self.queue.lock().unwrap().push_back(f);
                return Err(CustomError::NotStarted); // just store task in queue without notification? Hmm
            }
            State::RUNNING => {
                self.queue.lock().unwrap().push_back(f);
                self.cv.notify_all();
            }
            State::STOPPED => {
                return Err(CustomError::STOPPED);
            }
        }
        Ok(())
    }

    pub fn join(&mut self) {

    }
}

// impl Display for Executor {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl Drop for Executor {
//     fn drop(&mut self) {
//         todo!()
//     }
// }

unsafe impl Sync for Executor {}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc;
    use super::Executor;


    #[test]
    fn create_executor() {
        let mut executor = Executor::new();
        let mut f = || -> () {println!("Hello!")};
        executor.append(Box::new(f));
    }
}
