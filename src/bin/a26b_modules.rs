// Topic: Inline Modules
//
// Summary:
// The existing program is complete, but all the code exists
// in a single module. This code can benefit from being organized
// into multiple modules.
//
// Requirements:
// * Organize the code into two modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * After moving the functions into modules, try running
//   `cargo check --bin a26b` to get a listing of required code changes

mod msg {

    pub(crate) fn trim(msg: &str) -> &str {
        msg.trim()
    }

    pub(crate) fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
        if let Some(letter) = msg.get(0..1) {
            format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
        } else {
            msg.into()
        }
    }
    pub(crate) fn exciting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

mod math {

    pub(crate) fn add(lhs: isize, rhs: isize) -> isize {
        lhs + rhs
    }
    pub(crate) fn sub(lhs: isize, rhs: isize) -> isize {
        lhs - rhs
    }
    fn mul(lhs: isize, rhs: isize) -> isize {
        lhs * rhs
    }
}




fn main() {
    // Part 1: math functions
    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        mul(three, three)
    };

    // Ensure we have a correct result.
    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    // Part 2: string functions
    let hello = {
        let msg = "hello ";
        let msg = msg::trim(msg);
        msg::capitalize(msg)
    };
    let world = {
        let msg = "world";
        msg::exciting(msg)
    };
    let msg = format!("{}, {}", hello, world);

    // Ensure we have a correct result.
    assert_eq!(&msg, "Hello, world!");
    println!("{}", msg);
}
