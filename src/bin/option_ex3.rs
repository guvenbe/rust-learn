struct User{
    id : u32,
    name: String,
}

fn get_user_name(id: u32) -> Option<String> {
    let database = [
      User{id: 1, name: String::from("Alice")},
      User{id: 2, name: String::from("Bob")},
      User{id: 3, name: String::from("Cindy")}
    ];
    for user in database {
        if user.id == id {
            return Some(user.name)
        }
    }
    None
}
fn main() {
    let user_id = 3;
    if let Some(name) = get_user_name(user_id){
        println!("Users name: {name}");
    }
}

// 1) The <expression> on the righthand side of the equals sign is first evaluated and this returns some value
//
// 2) If the <pattern> on the lefthand side of the equals sign matches the value returned from <expression>, then the "if" statement evaluates to "true", and the coding in the block is performed.
//
// 3) Since "if let ..." is just a modified "if" statement, it is perfectly valid to include an "else" block in which you handle all cases where the pattern does not match.
//
// 4) In the case of testing an Option, there are only two possible outcomes: "Some<T>" and "None"
//
// 5) By only specifying what to do if the "if let ..." evaluates to "true", we are implicitly ignoring the only other possible outcome ("None")
//
// 6) Nonetheless, to state our intent explicitly (which is a foundational principle in Rust), it would be perfectly correct to include an "else" block in which we handle the "None" outcome.