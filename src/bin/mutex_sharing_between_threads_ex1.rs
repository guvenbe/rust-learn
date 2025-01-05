use std::collections::hash_map::Values;
use std::sync::{Arc, Mutex};
use std::thread;
struct Wrapper {
    value: i32,
}

impl Wrapper {
    fn new() -> Self {
        Wrapper { value: 0 }
    }
    fn add(&mut self, to_add: i32) {
        self.value += to_add;
    }
}
//calculate sum of range 1..40000 using 4 threads
fn main() {
    let sum = Arc::new(Mutex::new(Wrapper::new()));
    let mut handles = Vec::new();
    for i in 0..4 {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            let start = i * 10000 + 1;
            for num in start..start + 10000 {
                sum += num;
            }
            let mut lock = sum_clone.lock().unwrap();
            lock.add(sum);
        });
        handles.push(handle)
    }
    for handle in handles{
        handle.join().unwrap();
    }
    let lock = sum.lock().unwrap();
    println!("Sum of range 1..=40000 : {}", lock.value );
}