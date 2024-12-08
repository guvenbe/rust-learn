fn display_first_name<S: Into<String>>(fist_name: S) {
    const MAX: i32 = 99;
    let string_value = fist_name.into();
    println!("First Name: {}", string_value);
}
fn display_last_name<S: Into<String>>(last_name: S) {
    let string_value = last_name.into();
    println!("Last_Name: {}", string_value);
    println!("Last_Name: {}", string_value);
}
fn main() {
    display_first_name("Bora");
    display_last_name("Guven");
}
