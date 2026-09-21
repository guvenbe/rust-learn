// src/main.rs
include!(concat!(env!("OUT_DIR"), "/greeting.rs"));

fn main() {
    println!("{}", greet());
}