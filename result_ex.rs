#[derive(Debug)]
fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);
    match a {
        Result::Ok(v) => println!("val = {}", v),
        _ => {}
    }
    if let Result::Ok(v) = a {
        println!("val2 = {}", v);
    }
    println!("a={:?}, b={:?}", a, b)
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot Dvide by zero".to_string());
    }
    Res::Thing(a / b)
}
