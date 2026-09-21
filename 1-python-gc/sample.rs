//! ```cargo
//! [package]
//! name = "explicit"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

struct Data {
    buffer: Vec<u8>,
}

impl Data {
    fn new(size: usize) -> Self {
        println!("Data allocated: {} bytes", size);
        Data {
            buffer: vec![0; size],
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Data released (via Drop)");
    }
}

fn create_data() {
    let _d = Data::new(10 * 1024 * 1024);
    println!("Exiting create_data");
}

fn main() {
    create_data();
    println!("Back in main");
}