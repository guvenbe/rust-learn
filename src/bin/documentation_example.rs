/// A favorite color
enum Color {
    Red,
    Blue,
}

///A peice of mail
struct Mail {
    address: String,
}

///Adds two number together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    add(1,3);
    println!("{}", "bora".to_uppercase());
}
//cargo doc --open  --bin documentation_example 