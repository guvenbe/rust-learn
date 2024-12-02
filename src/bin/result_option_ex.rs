#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}
fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);
    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }
    if let Res::Thing(v) = a {
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
