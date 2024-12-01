fn main() {
    let mut s1 = String::from("Rust");
    print_string(&s1);
    println!("{s1}");
    add_to_string(&mut s1);
    println!("{:?}", s1);
    let s2 = generate_string();
}

fn print_string(p1: &String) {
    println!("{}", p1);
}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome");
    //(*p1).push_str("is awesome");
}

fn generate_string() -> String {
    String::from("Ferris")
}
