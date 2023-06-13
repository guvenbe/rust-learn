fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn conditional<'a>(n: i32) -> &'a str {
    if n > 200 {
        "Huge Number"
    } else if n > 99 {
        "Big Number"
    } else {
        "Small Number"
    }
}

fn my_loop() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a += 1;
    }
}

fn my_while_loop() {
    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a += 1;
    }
}
fn my_while_loop2() {
    let mut a = 5;
    while a > 0 {
        println!("{:?}", a);
        a -= 1;
    }
    println!("Done!!!!");
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    println!("Hello, world!");
    let x = add(1, 1);
    let y = add(3, 0);
    let z = add(x, 1);
    println!("{} {} {} ", x, y, z);
    let result = conditional(99);
    println!("result {}", result);
    my_loop();
    my_while_loop();
    my_while_loop2();
    let five = sub(8, 3);
    println!("{}", five);
}
