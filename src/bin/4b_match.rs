// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {

    let i: i32 = 2;

    match i {
        1 =>println!("one"),
        2 =>println!("two"),
        3 =>println!("three"),
        _ =>println!("Some other number"),
    }
}