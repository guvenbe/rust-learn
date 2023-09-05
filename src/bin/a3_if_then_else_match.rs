fn tester(b: bool) {
    if b == true {
        println!("{}", "Hello");
    } else {
        println!("{}", "goodbye");
    }
}

fn tester2(b: i32) {
    if b < 5 {
        println!("{}", "<5");
    } else if b > 5 {
        println!("{}", ">5");
    } else {
        println!("{}", "==5");
    }
}

fn tester_match(i: i32) {
    match i {
        1 => println!("{}", "it is 1"),
        2 => println!("{}", "it is 2"),
        3 => println!("it is 3"),
        _ => println!("{}", "Something else"),
    }
}

fn test_bool(b: bool) {
    match b {
        true => println!("It's true"),
        false => println!("It's ifalse"),
    }
}

fn tester_match_string<S: Into<String>>(name: S) {
    let my_str = name.into();
    match &my_str[..] {
        "Jayson" => println!("Hello {}", my_str),
        "Bob" => println!("Not my name"),
        "Alice" => println!("hi {}", my_str),
        _ => println!("nice to meet you"),
    }
}

fn main() {
    tester(true);
    tester2(3);
    tester_match(2);
    tester_match_string("Jayson");
    tester_match_string("Bob");
    tester_match_string("Rob");
    test_bool(true);
}
