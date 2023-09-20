fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("divide by zero error");
    }
    a / b
}

fn main() {
    let _res = div(23, 0);
}