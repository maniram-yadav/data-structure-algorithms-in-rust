use std::sync::atomic::{AtomicUsize,Ordering};
use std::sync::{Arc};
use std::thread;
use std::time::Duration;

struct LockFreeCounter {
    count : AtomicUsize,
    successful_increments : AtomicUsize,
}    

impl LockFreeCounter {

    fn new() -> Self {

        Self {
            count:AtomicUsize::new(0),
            successful_increments:AtomicUsize::new(0),
        }
    }

    fn increment(&self) -> bool {
        
        let mut current = self.count.load(Ordering::Relaxed);
        loop {
             if current >= 1000 { 
                return false;
            }

            match self.count.compare_exchange_weak(
                current,
                current+1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {

                Ok(_) => {
                    self.successful_increments.fetch_add(1,Ordering::Relaxed);
                    return true;
                }
                Err(actual) => {
                    current = actual ;
                }
            }
        }
    }
}


mod test {

    use super::*;
    #[test]
    fn atomic_operations(){
            let counter = Arc::new(LockFreeCounter::new());

            let handles: Vec<_> = (0..10)
                    .map(|i| { 
                        let counter_clone =  Arc::clone(&counter);
                        thread::spawn(move ||{
                                while counter_clone.increment(){
                                    thread::sleep(Duration::from_millis(10));
                                }
                        })

                    }).collect();

             
             for handle in handles {
                handle.join().unwrap();
             }      

             println!("Count : {} , Successfull Increments : {}",counter.count.load(Ordering::SeqCst),
                                        counter.successful_increments.load(Ordering::SeqCst));
    }
}