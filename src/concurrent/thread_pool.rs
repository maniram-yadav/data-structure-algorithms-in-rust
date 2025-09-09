use std::sync::{Arc,Mutex,Condvar};
use std::thread;
use std::time::Duration;

type Job = Box<dyn FnOnce() + Send + 'static>;


struct ThreadPool{
    workers : Vec<Worker>,
    sender : crossbeam::channel::Sender<Job>,
}

impl ThreadPool {

        fn new( size:usize) -> Self{
            assert!(size>0);

            let (sender,receiver) = crossbeam::channel::unbounded();
            let receiver_arc = Arc::new(receiver);

            let workers = (0..size)
                    .map(|id| Worker::new(id,Arc::clone(&receiver_arc)))
                    .collect();

            Self {
                workers,
                sender
            }

        }

        fn execute<F>(&self , f : F) 
            where  
                F : FnOnce() + Send + 'static , {

                    self.sender.send(Box::new(f)).unwrap();
                } 
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {

    fn new(id:usize, receiver : Arc<crossbeam::channel::Receiver<Job>>) -> Self {
        let th = thread::spawn(move ||{
            while let Ok(job) = receiver.recv() {
                    println!("Worker {} got a job, Processing it",id);
                    job();
            }
        });

        Self {
            id,
            thread: Some(th)
        }
    }


}

impl Drop for ThreadPool {

        fn drop(&mut self) {
            for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                             thread.join().unwrap();
                }           
            }
        }
}


mod test {

    use super::*;
    // #[test]
    fn test_thread_pool(){
            let pool =  ThreadPool::new(3);
            println!("Thread Pool created .");
            for i in 0..8 {
                pool.execute(move || {
                    println!("Task {} executing in thread",i);
                    thread::sleep(Duration::from_millis(500));
                    println!("Task {} completed",i);

                });
            }

            thread::sleep(Duration::from_secs(4));
    }
}