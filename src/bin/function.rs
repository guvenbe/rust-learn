fn main() {
    let z = my_function(50, 20);
    println!("my_function returned {}", z);
    call_me(9395550113);
}

fn my_function(x: i32, y: i32) -> i32{
    println!("my_function is called with {} and {}", x, y);
    let y = 10;
    y
}

fn call_me(num: i64) {
    println!("Ring! Call number {}", num);
}