//! ```cargo
//! [package]
//! name = "niche"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::mem;

fn main() {
    println!("Size of &u8: {}", mem::size_of::<&u8>());
    println!("Size of Option<&u8>: {}", mem::size_of::<Option<&u8>>());
}