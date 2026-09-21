//! ```cargo
//! [package]
//! edition = "2021"
//! ```

fn call_twice<F>(f: F) -> usize

where
    // same as F: for<'a> Fn(&'a str) -> usize
    F: Fn(&str) -> usize,
{
    let s1 = "hello";               // &'static str
    let s2 = String::from("world"); // shorter lifetime
    f(s1) + f(&s2)
}

fn main() {
    println!("Result: {}", call_twice(|s| s.len()));
}