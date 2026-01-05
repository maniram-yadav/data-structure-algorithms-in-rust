

//--- START OF FILE: main_futures.rs ---


use tokio;
use tokio::time::{sleep, Duration};
use futures::future;

async fn my_async_task(id:u64){
     sleep(Duration::from_millis(100*id)).await;
    println!("Task {} completed",id);
}

#[tokio::main]
async fn main() {
    let mut tasks = Vec::new();
    for i in 1..=10 {
        tasks.push(my_async_task(i));
    }
    future::join_all(tasks).await;
    println!("All task completed.");
   
}

//--- END OF FILE: main_futures.rs ---
