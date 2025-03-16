use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// The core idea is 
///     Thread pool writes to a channel
///     All threads in the pool compete to read from the channel
///     The thread that get the new closure will run it
/// 
/// In order to allow multi-ownership in multi-thread background, we need Arc<Mutex>
/// 
/// In order to automatically drop, we need to implement `Drop` Trait, but it takes a 
/// `&mut`, while we need to consume the sender, thus we use the pair of `Option` and `take()`

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let recv = Arc::new(Mutex::new(rx));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&recv)));
        }

        ThreadPool {
            workers,
            sender: Some(tx),
        }
    }

    pub fn execute<T>(&self, f: T)
    where 
        T: FnOnce() + Send + 'static
    {
        let f = Box::new(f);
        self.sender.as_ref().unwrap().send(f).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // Because we need ownership to call `join()`, and it is just a mutable reference
        // We can use `Option` and `take()` agian
        // But a better choice is `drain()`, it consumes the range and returns iterator
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

impl Worker {
    pub fn new(id: usize, recv: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Worker {
            id,
            // When the tx is freed, all f will be Err(_), then exit
            thread: thread::spawn(move || loop {
                let f = recv.lock().unwrap().recv();
                match f {
                    Ok(job) => {
                        println!("thread {} starts a new job", id);
                        job();
                    },
                    Err(_) => {
                        println!("thread {} is about to exit", id);
                        break;
                    }
                }
            }),
        }
    }
}