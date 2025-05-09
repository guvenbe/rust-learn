use std::thread;
use std::time::Duration;

fn main() {
    let value = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });
    println!("Waiting on thread");
    match value.join() {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("error joining thread: {:?}", e)
    }

    println!("waiting on thread");
}
