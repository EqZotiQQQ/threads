use std::collections::VecDeque;
// use std::fmt::{Display, Formatter};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Debug, Clone, Copy)]
enum State {
    // NEW,
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
    queue: Arc<Mutex<VecDeque<Box<dyn Fn() + Send + 'static>>>>,
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            executor: Some(
                thread::spawn(|| {
                    loop {}
                })
            ),
            state: State::RUNNING,
            cv: Condvar::new(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // Dumb shit is to use dyn Fn() + Send. It's literally illogical syntax.
    // Box<dyn Fn()> sugar for Box<dyn Fn() + 'static>
    pub fn append(&mut self, f: Box<dyn Fn() + Send>) -> Result<(), CustomError> {
        // let s = self.queue.clone();
        // let s = f;
        // let processor = move || {
        //     loop {
        //         // f();
        //         // s.lock().unwrap().len();
        //         // let s = self.queue.clone();
        //         // if s.lock().unwrap().is_empty() {
        //         if self.queue.lock().unwrap().is_empty() {
        //             self.cv.wait(self.queue.lock().unwrap()).unwrap();
        //         } else {
        //         //     let f = self.queue.lock().unwrap().pop_back().unwrap();
        //         //     f();
        //         }
        //         // println!("Some stupid words! {}", s.lock().unwrap().len());
        //     }
        // };
        match self.state {
            // State::NEW => {
            //     self.state = State::RUNNING;
            //     self.queue.lock().unwrap().push_back(f);
            // }
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
