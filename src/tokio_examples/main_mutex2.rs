


//--- START OF FILE: main_mutex2.rs ---


use tokio;
use std::sync::{Arc};
use tokio::sync::{Mutex};
use tokio::time::{sleep, Duration};
use futures::future;
use rand::Rng;

async fn person(remote_arc : Arc<Mutex<i32>>, name :String , new_channel : i32){
       
        sleep(Duration::from_secs(1)).await;
        let mut real_remote = remote_arc.lock().await;
        *real_remote = new_channel;
        
        sleep(Duration::from_secs(1)).await;
        println!("{} changes the channel",name);
        println!("Watching channel {}",*real_remote);

}

#[tokio::main]
async fn main() {
    
      let mut rng = rand::thread_rng();
      let tv_channel = 10;
      let remote = Mutex::new(tv_channel);
     let channel_arc = Arc::new( Mutex::new(tv_channel));
    
    let mut handlers = Vec::new();
    for i in 1..=10 {
        let name = format!("Person - {}", i);
        let channel_clone = Arc::clone(&channel_arc);
        let channel = rng.gen_range(1..=100);
        handlers.push(
            tokio::spawn(async  move {
                person(channel_clone, name, channel).await;
            }));
    }
    future::join_all(handlers).await;
    println!("last channel : {} ",*channel_arc.lock().await );
   
}

//--- END OF FILE: main_mutex2.rs ---
