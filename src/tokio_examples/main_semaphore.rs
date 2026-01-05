
//--- START OF FILE: main_semaphore.rs ---


use tokio;
use std::sync::{Arc};
use tokio::sync::{Semaphore};
use tokio::time::{sleep, Duration};
use futures::future;
use rand::Rng;

async fn random_wait(){
    let wait = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=200)
    };
    sleep(Duration::from_millis(30*wait)).await;
}

async fn person(semaphore : Arc<Semaphore>,name:String){
    println!("{} is waiting for line",name);
    teller(semaphore,name).await;
}
async fn teller(semaphore:Arc<Semaphore>,name :String){
    let wait =  40;
    let permit = semaphore.acquire().await.unwrap();
    random_wait().await;
    // sleep(Duration::from_millis(50*wait)).await;
    println!("{} is being served by the teller",name);
    // sleep(Duration::from_millis(30*wait)).await;
    random_wait().await;
    println!("{} is now leaving the teller",name);
    drop(permit);
}

#[tokio::main]
async fn main() {
    let num_of_tellers = 4;
    let semaphore = Semaphore::new(num_of_tellers);
    let semaphore_arc = Arc::new(semaphore);   
    
    let mut people_handles = Vec::new();
    for num in 0..10 {
        let semaphore_clone = Arc::clone(&semaphore_arc);
        people_handles.push(
                tokio::spawn(
                    async move {
                    person(
                        semaphore_clone,
                        format!("Person - {}",num),
                    ).await;
                }));
    }

    for handle in people_handles {
        handle.await.unwrap();
    }
    
    // future::join_all(people_handles).await;

    println!("All people have been served");


   
}

//--- END OF FILE: main_semaphore.rs ---
