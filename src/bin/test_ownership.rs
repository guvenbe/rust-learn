#![allow(unused_mut)]
fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("please supply an argument to this program");
        std::process::exit(-1);
    });

    inspect(&arg);
    println!("my arg is {}", arg);

    change(&mut arg);
    println!("I have many apples {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not Bananas");
    }
}

// Take mutable reference
fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push('s');
    }
}

//Takes immutable reference
fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is prulal", s);
    } else {
        println!("{} is singular", s)
    }
}

//Takes ownership of (consumes)

fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}
