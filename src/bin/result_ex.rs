fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);
    match a {
        Ok(v) => println!("val = {}", v),
        _ => {}
    }
    if let Ok(v) = a {
        println!("val2 = {}", v);
    }
    println!("a={:?}, b={:?}", a, b)
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot Dvide by zero".to_string());
    }
    Result::Ok(a / b)
}
