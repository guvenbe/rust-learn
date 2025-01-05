use std::sync::{Arc, Mutex};
use std::thread;
fn is_prime(num: u32) -> bool {
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    if num <= 1 {
        false
    } else {
        true
    }
}

fn main() {
    let mut primes = Arc::new(Mutex::new(Vec::new()));
    let thread_count = 4;
    let elememnts_per_thread = 10000 / thread_count;
    let mut handles = Vec::new();
    for i in 0..thread_count {
        let start = i * elememnts_per_thread;
        let list_clone = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            for num in start..start + elememnts_per_thread {
                if is_prime(num) {
                    let mut lock = list_clone.lock().unwrap();
                    lock.push(num);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let lock = primes.lock().unwrap();
    println!("prime numbers:");
    println!("{:?}", lock);
    //assert_eq!(lock.len(), 1229);
}
