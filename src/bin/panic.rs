fn main() {
    let v = vec!["one", "two", "three"];
    println!("{}", v[3]);
}

// RUST_BACKTRACE=1 cargo run --bin panic
// RUST_BACKTRACE=full cargo run --bin panic 