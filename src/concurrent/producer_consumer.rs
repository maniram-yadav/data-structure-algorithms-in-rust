use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex,Condvar};
use std::collections::VecDeque;


struct BoundedBuffer<T> {
    buffer : Mutex<VecDeque<T>>,
    capacity : usize,
    not_full: Condvar,
    not_empty: Condvar,
}


impl<T> BoundedBuffer<T> {

    pub fn new(capacity:usize) -> Self {
        Self {
            buffer : Mutex::new(VecDeque::<T>::new()),
            capacity : capacity,
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    pub fn push(&self,item :T){

        let mut buffer = self.buffer.lock().unwrap();
        
        while buffer.len() == self.capacity {
            buffer = self.not_full.wait(buffer).unwrap();
        }

        buffer.push_back(item);
        self.not_empty.notify_one();


    }


    pub fn pop(&self) -> T {
        let mut buffer = self.buffer.lock().unwrap();
        while buffer.is_empty(){
            buffer = self.not_empty.wait(buffer).unwrap();
        }
        let item =  buffer.pop_front().unwrap();
        self.not_full.notify_one();
        item

    }

    pub fn producer_consumer(){

        let buffer = Arc::new(BoundedBuffer::new(5));

        let producer = {
            let buffer = Arc::clone(&buffer);
            thread::spawn(move || {

                for i in 1..20 {
                    buffer.push(i);
                    println!("Produced : {}",i) ;
                    thread::sleep(Duration::from_millis(125));   
                }
            })
        };

        let consumer = {
            let buffer = Arc::clone(&buffer);
            thread::spawn(move || {
                for _ in 1..20 {
                    let item = buffer.pop();
                    println!("Consumed {}",item);
                    thread::sleep(Duration::from_millis(78));
                }
            })
        };

        producer.join().unwrap();
        consumer.join().unwrap();


    }
}

mod test {
    use super::*;


// #[test]
fn test_producer_consumer_runs_without_panic() {
            BoundedBuffer::<i32>::producer_consumer();
    }
}