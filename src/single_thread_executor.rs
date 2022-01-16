use std::collections::VecDeque;
// use std::fmt::{Display, Formatter};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
    NEW,
    RUNNING,
    STOPPED,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomError {
    NotStarted, // Invoked when you append functions while Executor didn't start yet
    STOPPED,    // Invoked when you append functions while Executor Stopped
    AlreadyRunning, // Invoked when you tried to start Executor twice
}

pub struct Executor {
    executor: Option<thread::JoinHandle<()>>,
    state: State,
    cv: Arc<Condvar>,
    queue: Arc<Mutex<VecDeque<Box<dyn Fn() + Send>>>>,
}

/**
Stupid executor for no-return functions
 */
impl Executor {
    pub fn new() -> Executor {
        Executor {
            executor: None,
            state: State::NEW,
            cv: Arc::new(Condvar::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())), // maybe replace vecdequeue to channel or crossbeam channel // 1st many to one 2nd many to many
        }
    }

    /// Starts executor
    pub fn start(&mut self) -> Result<(), CustomError>{
        match self.state {
            State::NEW => {
                let queue = Arc::clone(&self.queue);
                let cv = Arc::clone(&self.cv);
                self.executor = Some(thread::spawn(move || {
                    loop {
                        let mut lock = queue.lock().unwrap();
                        if lock.is_empty() {
                            cv.wait(lock).unwrap();
                        } else {
                            let f = lock.pop_back().unwrap();
                            f();
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
    /// Append function to queue
    pub fn append(&mut self, f: Box<dyn Fn() + Send + 'static>) -> Result<(), CustomError> {
        match self.state {
            State::NEW => {
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

    /// Finish executor
    pub fn join(&mut self) -> Result<(), CustomError> {
        self.state = State::STOPPED;
        let mut queue = Arc::clone(&self.queue);
        while !self.queue.lock().unwrap().is_empty() {
            let lock = queue.lock().unwrap();
            self.cv.wait(lock).unwrap();
        }
        match &mut self.executor {
            None => {
                return Err(CustomError::NotStarted);
            }
            Some(handler) => {
                // handler.join();
            }
        }
        Ok(())
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
