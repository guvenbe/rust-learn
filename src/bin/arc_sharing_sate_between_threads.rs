use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Database {
    connections: Vec<i32>,
}
impl Database {
    fn new() -> Self {
        Self {
            connections: vec![],
        }
    }
    fn connect(&mut self, id: i32) {
        self.connections.push(id);
    }
}
fn main() {
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];
    for i in 1..10 {
        let db_clone = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db = db_clone.lock().unwrap();
            db.connect(i);
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
    let db_lock = db.lock().unwrap();
    println!("{db_lock:?}")
}
