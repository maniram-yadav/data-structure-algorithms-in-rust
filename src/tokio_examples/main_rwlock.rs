
//--- START OF FILE: main_rwlock.rs ---



use tokio;
use std::sync::{Arc};
use tokio::sync::{RwLock};
use tokio::time::{sleep, Duration};
use rand::Rng;

async fn random_wait(){
    let wait = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=200)
    };
    sleep(Duration::from_millis(30*wait)).await;
}

async fn read_from_document(id:i32,document : Arc<RwLock<String>> )  {
        
    let reader = document.read().await;
    println!("Reader {} read document {}",id,reader);

}
async fn write_to_document(new_str:&str,document : Arc<RwLock<String>> )  {
        
    let mut writer = document.write().await;
    writer.push_str(new_str);
    writer.push_str(" ");
    println!("Writer wrote document {}",writer);
    
}


#[tokio::main]
async fn main() {

    let document = Arc::new(RwLock::new("".to_string()));    
    let mut handles = vec![];
    for new_str in  "I can read this d o i yb db r g tn f d nm ki y erc gfdes e".split_whitespace(){
       
        handles.push(tokio::spawn(read_from_document(1,document.clone() )));
        handles.push(tokio::spawn(write_to_document(new_str,document.clone()) ));
        handles.push(tokio::spawn(read_from_document(2,document.clone() )));
        handles.push(tokio::spawn(read_from_document(3,document.clone() )));
    }

    for handle in handles {
        handle.await.unwrap();
    }
    println!("All task completed");
    
}

//--- END OF FILE: main_rwlock.rs ---
