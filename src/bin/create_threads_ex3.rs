use std::{thread, time::Duration};
fn main() {
    thread::spawn(|| {
        println!("coundown down from 10 to 1 🚀");
        for num in (1..10).rev() {
            println!("count down: {num}");
            thread::sleep(Duration::from_millis(100));
        }
    });
    println!("count up from 1 to 10 ....");
    
    for num in 1..=10 {
        println!("count up {num}");
    }
    thread::sleep(Duration::from_millis(500));
}
