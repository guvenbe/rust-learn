enum Color {
    Red, Blue,Green
}
fn main() {
    let maybe_user = Some("Jerry");
    
    match maybe_user {
        Some(user) => println!("user={:?}", user),
        None => println!("no user")
    }
    
    if let Some(user) = maybe_user{
        println!("user={:?}", user);
    }
    let red = Color::Red;
    
    if let Color::Red = red{
        println!("its red");
    }else {
        println!("It's not red");
    }
}