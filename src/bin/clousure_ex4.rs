// Make the code compile by addressing the TODO.

fn main() {
    let my_str = "Hi there!".to_owned();
    let substr = "here";
    println!("String: {my_str}");
    let check_substr = move |sub: &str| my_str.contains(sub);
    let res = check_substr(substr);
    // TODO: shift the statement below to somewhere else
    println!("String contains {substr} : {res}");
}