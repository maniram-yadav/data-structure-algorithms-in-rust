
//--- START OF FILE: main_basics.rs ---


use tokio;
use tokio::time::{sleep, Duration};

async fn async_fn(){
     sleep(Duration::from_secs(4)).await;
    println!("Msg from async function");
}

#[tokio::main]
async fn main() {
    
    let handler1 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task1 finishes");

    });

    let handler2 = tokio::spawn( async {
        println!("Tokio 2 started immediately!");
    });

    let handler3 = tokio::spawn( async {

       async_fn().await;
    }
    );

    handler1.await.unwrap();
    handler2.await.unwrap();
    handler3.await.unwrap();
    async_fn().await;
    println!("Async code completed");
}

//--- END OF FILE: main_basics.rs ---
