use std::{sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

type Job = Box<dyn FnOnce() + Send + 'static>;

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