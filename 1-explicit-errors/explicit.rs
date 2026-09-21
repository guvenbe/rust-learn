//! ```cargo
//! [package]
//! name = "explicit"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct User {
    name: String,
}

#[derive(Debug)]
enum UserError {
    NotFound,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User not found"),
        }
    }
}

impl Error for UserError {}

fn find_user_by_id(id: u32) -> Result<User, UserError> {
    if id == 1 {
        Ok(User { name: "Alice".to_string() })
    } else {
        Err(UserError::NotFound)
    }
}

fn run() -> Result<(), UserError> {
    let user = find_user_by_id(42)?; // early return if error
    println!("User found: {}", user.name);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
    }
}