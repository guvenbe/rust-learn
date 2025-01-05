use tokio::time::sleep;
use std::time::Duration;

// #[tokio::main]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut handles = vec![];
    for i in 0..2{
        let handle = tokio::spawn(async move{
           my_function(i).await; 
        });
        handles.push(handle);
    }
    
    for handle in handles{
        handle.await.unwrap();
    }
    
}

async fn my_function(i: u32){
    println!("I'm a async function");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}")
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_owned()
}