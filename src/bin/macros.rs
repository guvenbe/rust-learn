// macro_rules! name {
//     rule0 ;
//     rule1 ;
//     // …
//     ruleN ;
// }

// (matcher) => { expansion aka transcriber }
// Fix the code to make it compile.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}