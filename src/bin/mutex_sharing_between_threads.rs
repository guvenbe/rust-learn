use std::sync::Mutex;
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
    let db = Mutex::new(Database::new());
    {
        let mut db_lock = db.lock().unwrap();
        db_lock.connect(1);
        //lock is atumatically dropped
    }
}
