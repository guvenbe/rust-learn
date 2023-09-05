fn main() {
    let mut count = 3;
    loop {
        println!("{:?}", count);
        count -= 1;
        if count == 0 {
            break
        }
    }
    println!("done!...")
}