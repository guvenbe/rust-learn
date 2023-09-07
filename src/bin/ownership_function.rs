fn main() {
    let s1 = String::from("Rust");
    print_string(s1); //owner ship is passed to print_string, the way to fix is to clone
    //println!(s1); //this will fail because s1 ownership is moved

    let s3 =generate_string(); //owner ship is transferred to s3
    println!("{}", s3);

    let s4 =add_to_string(s3);
    println!("{s4}");
    let i =3;
    print_integer(3); //i is cloned
    println!("{}", i); //so this still works

}

fn print_string(p1: String){
    println!("{p1}");

} //p1 is dropped

fn generate_string() -> String{
    String::from("ferris")
}

fn add_to_string(mut p1: String) -> String{
    p1.push_str(" is awesome");
    p1
}

fn print_integer(i: i32){
    println!("{i}");
}