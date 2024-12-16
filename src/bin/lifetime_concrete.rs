fn main() {
    let mut s1 = String::from("Let's get Rusty");
    //Borrow muttable reference
    let r2 = &mut s1;
    r2.push_str("!");
    let r1 = &s1;
    println!("r1:{r1}");
}
