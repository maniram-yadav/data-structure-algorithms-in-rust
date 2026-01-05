

//--- START OF FILE: main_barrier.rs ---


use tokio;
use std::sync::{Arc};
use tokio::sync::{Barrier,BarrierWaitResult, Notify};
use tokio::time::{sleep, Duration};
use rand::Rng;

async fn random_wait(){
    let wait = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=200)
    };
    sleep(Duration::from_millis(30*wait)).await;
}

async fn barrier_example(barrier:Arc<Barrier>,notify : Arc<Notify>) -> BarrierWaitResult {
        
        println!("Waiting for Barrier");
        let wait_result = barrier.wait().await;
        println!("Barrier passed");

        if wait_result.is_leader(){
            notify.notify_one();
        }
        wait_result

}


#[tokio::main]
async fn main() {

    let total_cans_needed = 12;
    let barrier = Arc::new(Barrier::new(total_cans_needed));
    let notify = Arc::new(Notify::new());
    let mut handles = vec![];
    notify.notify_one();

    for count in 0..50 {
        if count%total_cans_needed== 0 {
            notify.notified().await;
            println!("All cans collected, proceeding to order");
            random_wait().await;
        }
        handles.push(tokio::spawn(
                barrier_example(barrier.clone(),notify.clone()
        )));
    }

    let mut  num_leaders = 0;

    for handle in handles {

        let wait_result = handle.await.unwrap();
        if wait_result.is_leader(){
            num_leaders += 1;
        }

    }

    println!("Total number of leaders {}",num_leaders);

}

//--- END OF FILE: main_barrier.rs ---
