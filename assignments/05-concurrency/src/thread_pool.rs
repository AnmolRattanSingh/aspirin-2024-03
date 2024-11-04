use crate::error::ThreadPoolError;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

pub struct ThreadPool<T: Send + 'static> {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job<T>>>,
    receiver: mpsc::Receiver<T>,
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl<T: Send + 'static> ThreadPool<T> {
    pub fn new(num_threads: usize) -> Result<ThreadPool<T>, ThreadPoolError> {
        if num_threads == 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }

        let (job_sender, job_receiver) = mpsc::channel::<Job<T>>();
        let job_receiver = Arc::new(Mutex::new(job_receiver)); // Wrap in mutex so that multiple threads can access

        let (result_sender, result_receiver) = mpsc::channel::<T>();

        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let job_receiver = Arc::clone(&job_receiver);
            let result_sender = result_sender.clone();

            let thread = thread::spawn(move || loop {
                let job = {
                    let receiver = job_receiver.lock().unwrap();
                    match receiver.recv() {
                        Ok(job) => job,
                        Err(_) => break, // Exit loop when sender is dropped
                    }
                };

                let result = job();
                if result_sender.send(result).is_err() {
                    break;
                }
            });

            workers.push(Worker { thread });
        }

        Ok(ThreadPool {
            workers,
            sender: Some(job_sender),
            receiver: result_receiver,
        })
    }

    /// Execute the provided function on the thread pool
    pub fn execute<F>(&mut self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        match &self.sender {
            Some(sender) => {
                let job = Box::new(f);
                match sender.send(job) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ThreadPoolError::Send),
                }
            }
            None => Err(ThreadPoolError::Send),
        }
    }

    /// Close the thread pool, signaling that no more tasks will be sent
    pub fn close(&mut self) {
        self.sender.take();
    }

    /// Retrieve all results
    pub fn get_results(&self) -> Vec<T> {
        let mut results = Vec::new();
        while let Ok(result) = self.receiver.recv() {
            results.push(result);
        }
        results
    }
}

impl<T: Send + 'static> Drop for ThreadPool<T> {
    fn drop(&mut self) {
        self.close();
        for worker in self.workers.drain(..) {
            worker.thread.join().expect("Failed to join worker thread");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_zero_threads() {
        let result = ThreadPool::<i32>::new(0);
        assert_eq!(
            match result {
                Err(ThreadPoolError::ZeroThreads) => true,
                _ => false,
            },
            true
        );
    }

    #[test]
    fn test_execute_and_get_results() {
        let mut pool = ThreadPool::new(4).unwrap();

        // Execute some test jobs
        for i in 0..10 {
            let job = move || {
                thread::sleep(Duration::from_millis(10));
                i
            };
            pool.execute(job).unwrap();
        }

        // Close the pool to signal no more tasks will be sent
        pool.close();

        // Retrieve results
        let results = pool.get_results();
        assert_eq!(results.len(), 10);

        // Results might come in any order, so we sort them for comparison
        let mut results = results;
        results.sort();
        assert_eq!(results, (0..10).collect::<Vec<i32>>());
    }

    #[test]
    fn test_concurrent_execution() {
        let mut pool = ThreadPool::new(4).unwrap();
        let start = std::time::Instant::now();

        // Execute 4 jobs that each sleep for 100ms
        for _ in 0..4 {
            pool.execute(|| {
                thread::sleep(Duration::from_millis(100));
                true
            })
            .unwrap();
        }

        // Close the pool to signal no more tasks will be sent
        pool.close();

        // Retrieve results
        let results = pool.get_results();
        let elapsed = start.elapsed();

        assert_eq!(results.len(), 4);
        assert!(elapsed < Duration::from_millis(200)); // Should take less than 200ms since we are using 4 threads
    }
}
