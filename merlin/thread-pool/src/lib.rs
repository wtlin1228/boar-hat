#![warn(missing_docs, missing_debug_implementations)]

//! A simple thread pool implementation.
//!
//! This crate provides a thread pool for concurrent execution of jobs.
//!
//! # Example
//!
//! ```
//! # use thread_pool::ThreadPool;
//! let thread_pool = ThreadPool::new(4); // create a pool with 4 threads
//! thread_pool.execute(|| println!("executed by one of the worker"));
//! ```

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// A thread pool that manages a fixed number of worker threads.
///
/// The `ThreadPool` struct allows concurrent execution of jobs by distributing
/// them among its worker threads.
///
/// # Example
///
/// ```
/// # use thread_pool::ThreadPool;
/// let thread_pool = ThreadPool::new(4); // create a pool with 4 threads
/// thread_pool.execute(|| println!("executed by one of the worker"));
/// ```
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    /// Shuts down the thread pool.
    ///
    /// This method joins all worker threads, gracefully terminating them.
    fn drop(&mut self) {
        // Need to drop the sender to close the channel to break the worker thread's loop.
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Creates a new `ThreadPool` with the specified number of threads.
    ///
    /// # Parameters
    ///
    /// * `size`: The number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The function will panic if `size` is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes a job by sending it to one of the threads in the thread pool.
    ///
    /// # Parameters
    ///
    /// * `f`: The job to execute. It must be `Send` and `'static`.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
