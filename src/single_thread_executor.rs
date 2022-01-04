use std::collections::VecDeque;
use std::env::Args;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crate::single_thread_executor::State::PRISTINE;

enum State {
    PRISTINE,
    RUN,
    FINISH,
}

#[derive(Debug)]
enum CustomError {
    STOPPED,
}

pub struct Executor {
    executor: Option<thread::JoinHandle<()>>,
    // queue: VecDeque<Arc<dyn Fn()>>,
    state: State,
    cv: Condvar,
    mtx: Mutex<VecDeque<Arc<dyn Fn()>>>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            // queue: VecDeque::new(),
            executor: None,
            state: PRISTINE,
            cv: Condvar::new(),
            mtx: Mutex::new(VecDeque::new()),
        }
    }
    
    pub fn append(&mut self, f: Arc<dyn Fn()>) -> Result<(), CustomError> {
        match self.state {
            State::PRISTINE => {
                self.executor = Some(thread::spawn(|| {
                    self.state = State::RUN;
                    while self.state == State::RUN {
                        if self.mtx.lock().expect("").is_empty() {
                            self.cv.wait_while(&self.cv);
                        }
                    }
                }));
                self.append(f);
            }
            State::RUN => {}
            State::FINISH => {
                return Err(CustomError::STOPPED);
            }
        }
        self.queue.push_back(f);
        self.cv.notify_all();
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
        println!("Join");
        // self.executor.join();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc;
    use super::Executor;


    #[test]
    fn create_executor() {
        let mut executor = Executor::new();
        let mut f = || -> () {println!("Hello!")};
        executor.append(Arc::new(f));
    }
}
