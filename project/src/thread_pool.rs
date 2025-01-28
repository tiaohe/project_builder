use std::collections::VecDeque;
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub enum RejectionPolicy {
    Discard,
    DiscardOldest,
    CallerRuns,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    job_queue: Arc<(Mutex<VecDeque<Job>>, Condvar)>,
    core_threads: usize,
    max_threads: usize,
    policy: RejectionPolicy,
    idle_timeout: Duration,
}

impl ThreadPool {
    pub fn new(core_threads: usize, max_threads: usize, policy: RejectionPolicy, idle_timeout: Duration) -> ThreadPool {
        assert!(core_threads > 0 && max_threads >= core_threads);

        let (sender, receiver) = mpsc::channel();
        let job_queue = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));

        let mut workers = Vec::with_capacity(max_threads);

        for id in 0..core_threads {
            workers.push(Worker::new(id, Arc::clone(&job_queue), receiver.clone(), idle_timeout));
        }

        ThreadPool {
            workers,
            sender,
            job_queue,
            core_threads,
            max_threads,
            policy,
            idle_timeout,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let mut job_queue = self.job_queue.0.lock().unwrap();

        if self.workers.len() < self.max_threads {
            let id = self.workers.len();
            let job_queue = Arc::clone(&self.job_queue);
            let receiver = self.sender.clone();
            self.workers.push(Worker::new(id, job_queue, receiver, self.idle_timeout));
        }

        if job_queue.len() < self.max_threads {
            job_queue.push_back(Box::new(f));
            self.job_queue.1.notify_one();
        } else {
            match self.policy {
                RejectionPolicy::Discard => println!("Task discarded."),
                RejectionPolicy::DiscardOldest => {
                    job_queue.pop_front();
                    job_queue.push_back(Box::new(f));
                }
                RejectionPolicy::CallerRuns => (f)(),
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, job_queue: Arc<(Mutex<VecDeque<Job>>, Condvar)>, receiver: mpsc::Receiver<Message>, idle_timeout: Duration) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = {
                let (lock, cvar) = &*job_queue;
                let mut job_queue = lock.lock().unwrap();

                while job_queue.is_empty() {
                    let result = cvar.wait_timeout(job_queue, idle_timeout).unwrap();
                    job_queue = result.0;

                    if result.1.timed_out() {
                        println!("Worker {} timed out and is terminating", id);
                        return;
                    }
                }

                job_queue.pop_front()
            };

            if let Some(job) = job {
                println!("Worker {} got a job; executing.", id);
                job();
            } else {
                break;
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4, 8, RejectionPolicy::DiscardOldest, Duration::from_secs(5));

    for i in 0..20 {
        pool.execute(move || {
            println!("Executing task {}", i);
            thread::sleep(Duration::from_secs(1));
        });
    }

    thread::sleep(Duration::from_secs(10));
}
