use std::thread;
use std::time::Duration;

fn main() {
    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data.iter().map(|c| c.to_ascii_uppercase()).collect();
        data
    });
    println!("Waiting on thread");
    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error joining thread: {:?}", e)
    }

    println!("waiting on thread");
}
