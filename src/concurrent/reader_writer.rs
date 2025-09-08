use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct SharedData {
    data: RwLock<String>,
    reader_count: RwLock<usize>,
}

impl SharedData {

    fn new() -> Self {
        SharedData {
            data: RwLock::new("Hi".to_string()),
            reader_count: RwLock::new(0),
        }
    }

    fn read(&self, reader_id: usize) -> String {
        {
            let mut count = self.reader_count.write().unwrap();
            *count += 1;
            println!("Reader {} started. Total readers {} ", reader_id, *count);
        }
        let data = self.data.read().unwrap().clone();

        {
            let mut count = self.reader_count.write().unwrap();
            *count -= 1;
            println!("Reader {} completed. Total readers {} ", reader_id, *count);
        }
        data
    }

    fn write(&self, writer_id: usize, new_data: &str) {
        println!("Writer {} waiting to write ...", writer_id);

        while *self.reader_count.read().unwrap() > 0 {
            thread::sleep(Duration::from_millis(100));
        }
        let mut data = self.data.write().unwrap();
        *data = new_data.to_string();
        println!("Writer {} wrote : {} ", writer_id, new_data);
    }

    fn reader_writer_pattern() {
        let shared_data = Arc::new(SharedData::new());

        let reader_handles: Vec<_> = (0..5)
            .map(|i| {
                let data_clone = Arc::clone(&shared_data);
                thread::spawn(move || {
                    for j in 0..3 {
                        let data = data_clone.read(i);
                        println!("Reader {} , Iteration {} : {}", i, j, data);
                        thread::sleep(Duration::from_millis(200));
                    }
                })
            })
            .collect();

        let writer_handle = {
            let data = Arc::clone(&shared_data);
            thread::spawn(move || {
                for j in 0..2 {
                    data.write(0, &format!("New data from writer {} ", j));
                    thread::sleep(Duration::from_millis(200));
                }
            })
        };

        for handle in reader_handles {
            handle.join().unwrap();
        }

        
        writer_handle.join().unwrap();
    }
}
