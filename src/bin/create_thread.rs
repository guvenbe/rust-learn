use std::{thread, time::Duration};
use std::time;

fn main (){
    let handle = thread::spawn(||{
        for i in 1..=20{
            println!("spawned thread{}:", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=10{
        println!("main thread {}", i);
        thread::sleep(Duration::from_millis(1))
    };
    //main thread waits for spawned thread
    handle.join().unwrap();

}