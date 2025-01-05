#[tokio::main]
async fn main() {
    my_function().await;
}

 async fn my_function() {
    println!("Async function")
}