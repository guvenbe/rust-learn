fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
fn main() {
    let name = "Jayson";
    let add = Box::new(move |a, b| {
        println!("adding numbers for {}!", name);
        a + b
    });
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| a * b);
    println!("{}", math(1,3, add));
    println!("{}", math(1,3, sub));
    println!("{}", math(1,3, mul));
}