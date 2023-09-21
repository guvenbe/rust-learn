// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let coord = get_tuple();

    let (x, y) = coord;

    if y > 5 {
        println!(" y is greater than 5");
    } else if y < 5 {
        println!(" y is less than 5");
    } else {
        println!("y is equal to 5");
    }
}

fn get_tuple() -> (i32, i32) {
    (2, 6)
}