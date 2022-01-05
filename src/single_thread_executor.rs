use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

enum State {
    NEW,
    RUNNING,
    STOPPED,
}

#[derive(Debug)]
pub enum CustomError {
    STOPPED,
}

pub struct Executor {
    executor: Option<thread::JoinHandle<()>>,
    state: State,
    cv: Condvar,
    queue: Arc<Mutex<VecDeque<Box<dyn Fn()>>>>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            executor: None,
            state: State::NEW,
            cv: Condvar::new(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    pub fn append(&mut self, f: Box<dyn Fn()>) -> Result<(), CustomError> {
        let processor = || {
            loop {
                // let s = self.queue.clone();
                // if self.queue.lock().unwrap().is_empty() {
                //     // self.cv.wait(self.queue.lock().unwrap()).unwrap();
                // } else {
                // //     let f = self.queue.lock().unwrap().pop_back().unwrap();
                // //     f();
                // }
                // println!("Some stupid words! {}", s.lock().unwrap().len());
            }
        };
        match self.state {
            State::NEW => {
                self.executor = Some(thread::spawn(processor));
                self.state = State::RUNNING;
                self.queue.lock().unwrap().push_back(f);
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
}

impl Display for Executor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        todo!()
    }
}

unsafe impl Send for Executor {}

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
