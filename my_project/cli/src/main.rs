use core::greet;

fn main() {
    let msg = greet("from CLI");
    println!("{:?}", msg);
}