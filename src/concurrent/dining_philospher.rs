use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Philospher {
    pub seq: u64,
  pub   name: String,
  pub  left_fork: Arc<Mutex<i32>>,
    pub right_fork: Arc<Mutex<i32>>,
}

impl Philospher {
    pub fn eat(&self,seq : u64) {
        let left = self.left_fork.try_lock();
        if left.is_err() {
            println!("{} failed to acquire left fork", self.name);
            thread::sleep(Duration::from_millis(self.seq*135));
            return;
        }
        println!("{} has acquired the left fork", self.name);
        thread::sleep(Duration::from_millis(self.seq*270 as u64));

        match self.right_fork.try_lock() {
            Ok(_) => {
                println!("{} is Eating!", self.name);
                thread::sleep(Duration::from_millis(self.seq*37));
                println!("{} has finished eating", self.name);
            }
            Err(_) => {
                println!("{} failed to acquire right fork", self.name);
                thread::sleep(Duration::from_millis(self.seq*115));
            }
        }
    }

    pub fn think(&self) {
        println!("{} is Thinking", self.name);
        thread::sleep(Duration::from_millis(self.seq*230));
    }

    pub fn dining_cycle(&self) {
        let chopsticks = (0..5).map(|_| Arc::new(Mutex::new(0))).collect::<Vec<_>>();
        let philosphers: Vec<_> = (0..5)
            .map(|i| {
                let left = chopsticks[i].clone();
                let right = chopsticks[(i + 1) % 5].clone();

                Philospher {
                    seq: (i+1) as u64,
                    name: format!("Philospher {}", i + 1),
                    left_fork: left,
                    right_fork: right,
                }
            })
            .collect();

        let handles: Vec<_> = philosphers
            .into_iter()
            .map(|p| {
                thread::spawn(move || {
                    for i in 0..5 {
                        if(i%2)==0 {
                            thread::sleep(Duration::from_millis(150));
                                
                                p.eat(p.seq);
                                p.think();
                        } else {
                                p.think();
                                p.eat(p.seq);
                        }
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

mod test{
    #[test]
    fn test_dining_philosopher() {
        let philosopher = super::Philospher {
            seq: 1,
            name: "Philosopher 1".to_string(),
            left_fork: std::sync::Arc::new(std::sync::Mutex::new(0)),
            right_fork: std::sync::Arc::new(std::sync::Mutex::new(0)),
        };
        philosopher.dining_cycle();
    }

}
