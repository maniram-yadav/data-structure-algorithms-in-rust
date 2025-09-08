use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Philospher {
  pub   name: String,
  pub  left_fork: Arc<Mutex<i32>>,
    pub right_fork: Arc<Mutex<i32>>,
}

impl Philospher {
    pub fn eat(&self) {
        let left = self.left_fork.try_lock();
        if left.is_err() {
            println!("{} failed to acquire left fork", self.name);
            thread::sleep(Duration::from_millis(100));
            return;
        }

        thread::sleep(Duration::from_millis(100));

        match self.right_fork.try_lock() {
            Ok(_) => {
                println!("{} is eating!", self.name);
                thread::sleep(Duration::from_millis(100));
                println!("{} has finished eating", self.name);
            }
            Err(_) => {
                println!("{} failed to acquire right fork", self.name);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    pub fn think(&self) {
        println!("{} is thinking", self.name);
        thread::sleep(Duration::from_millis(100));
    }

    pub fn dining_cycle(&self) {
        let chopsticks = (0..5).map(|_| Arc::new(Mutex::new(0))).collect::<Vec<_>>();
        let philosphers: Vec<_> = (0..5)
            .map(|i| {
                let left = chopsticks[i].clone();
                let right = chopsticks[(i + 1) % 5].clone();

                Philospher {
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
                    for _ in 0..5 {
                        p.eat();
                        p.think();
                    }
                })
            })
            .collect::<Vec<_>>();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}


