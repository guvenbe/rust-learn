use std::string;

fn main() {
    let _a: i16 = 45;
    let s1 = String::from("Hello");
    let s2: &str = "hello";
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    //tuples
    let t1: (i32, i32, i32) = (1, 2, 3);

    let t2: (i32, f32, &str) = (1, 5.0, "hello");

    let unit: () = ();
    println!("{:?}", arr);

    const MAX_PLAYERS: u16 = 10;
    static CASINO_NAME: &str = "Rusty Casino";
}
