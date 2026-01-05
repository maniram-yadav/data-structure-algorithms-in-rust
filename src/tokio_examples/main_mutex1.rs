
//--- START OF FILE: main_mutex1.rs ---


use tokio;
use std::sync::{Arc,Mutex};

#[tokio::main]
async fn main() {
    
    let shared_data = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 0..10 {
        let data_clone = Arc::clone(&shared_data);
        handles.push(
            tokio::spawn(async move {
                let mut data = data_clone.lock().unwrap();
                *data += 1;
                println!("task {} completed, data {} ",i,*data);
            })
        );
    }

    for handle in handles {
        handle.await.unwrap();
    }
    println!("Final shared data : {} ",*shared_data.lock().unwrap());
   
}

//--- END OF FILE: main_mutex1.rs ---
