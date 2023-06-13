fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn substract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    a / b
}
fn modulus(a: i32, b: i32) -> i32 {
    a % b
}
fn display(n: i32) {
    println!("result: {}", n);
}
fn main() {
    let result = add(6, 2);
    display(result);
    let result2 = substract(6, 2);
    display(result2);
    let result3 = multiply(6, 2);
    display(result3);
    let result4 = divide(6, 2);
    display(result4);
    let result5 = modulus(6, 2);
    display(result5);
}
