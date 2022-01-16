
#[cfg(test)]
mod tests {
    use crate::single_thread_executor::Executor;


    #[test]
    fn create_executor() {
        let mut executor = Executor::new();
        let mut f = || -> () {println!("Hello!")};
        executor.append(Box::new(f));
    }
}
