
//--- START OF FILE: main_notify.rs ---


use tokio;
use std::sync::{Arc};
use tokio::sync::{Notify};
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

async fn order_package(package_delivered : Arc<Notify>){
    sleep(Duration::from_secs(2)).await;
    println!("Find package in warehouse");
     sleep(Duration::from_secs(2)).await;
    println!("Ship package");
     sleep(Duration::from_secs(2)).await;
    println!("Package has been delivered");
    package_delivered.notify_one();
}
async fn grab_package(package_delivered : Arc<Notify>){
    println!("Waiting for package to be delivered");
    package_delivered.notified().await;
    println!("Package received, I am happy!");
}

#[tokio::main]
async fn main() {

    let package_delivered = Notify::new();
    let package_delivered_arc = Arc::new(package_delivered);

    let order_packge_handle = tokio::spawn(
        order_package(package_delivered_arc.clone())
    );

    let grab_package_handle = tokio::spawn(
        grab_package(package_delivered_arc.clone())
    );

    order_packge_handle.await.unwrap();
    grab_package_handle.await.unwrap();
    println!("All task completed"); 

    
   
}

//--- END OF FILE: main_notify.rs ---
