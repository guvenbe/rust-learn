fn main() {
    let mut str1 =String::from("modifable");
    let str2= String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
    str_ptr = &str2;
    println!("ptr currently points to {str_ptr}");
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
}
