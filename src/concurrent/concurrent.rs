use crate::concurrent::dining_philospher::*;
use std::sync::{Arc, Mutex};



pub struct ConcurrentTest;

impl ConcurrentTest {
    pub fn new() -> Self {
        ConcurrentTest
    }

    pub fn test_dining_philosopher(&self) {
        let philosopher = Philospher {
            seq : 1,
            name: "Philosopher 1".to_string(),
            left_fork: Arc::new(Mutex::new(0)),
            right_fork: Arc::new(Mutex::new(0)),
        };
        philosopher.dining_cycle();
    }

}

