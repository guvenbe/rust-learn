fn main() {
    let x = 10;
    let y = {
        let x = 5;
        x * 2
    };
    println!("x: {}, y: {}", x, y);
}