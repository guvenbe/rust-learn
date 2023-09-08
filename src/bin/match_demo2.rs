fn main() {
    let age = 35;

    match age {
        1 => println!("Happy first birthday!!"),
        13..=19 => println!("You are a teenager"),
        // _ =>println!("")
        x => println!("You are {x} years old!") // x is also a catch all value
    }
}