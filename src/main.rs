fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    println!("Hello, world!");
    let x = add(1, 1);
    let y = add(3, 0);
    let z = add(x, 1);
    println!("{} {} {} ", x, y, z);
}
