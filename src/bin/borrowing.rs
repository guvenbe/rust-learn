fn main() {
    let mut s1 = String::from("Rust");
    let r1 = &s1;
    print_string(r1); //r1 is not used later (non lexical lifetime
    let r2 = &mut s1;
    add_to_string( r2);
    println!("{s1}");

    let s2 = generate_string();
    println!("{s2}")
}

fn print_string(p1:  &String){
    println!("{p1}");

}

fn add_to_string(p1: &mut String) {
    p1.push_str(" is awesome"); //This is a auto de reference
    (*p1).push_str(" is awesome");
}

fn generate_string() ->String{
    String::from("ferris")
}
