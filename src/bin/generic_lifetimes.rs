
fn main() {
    let player1 = String::from("player1");
    let player2 = String::from("player2");

    let result = first_turn(player1.as_str(), player2.as_str());

    println!("player going first is {}", result);
}

fn first_turn<'a>(p1: & 'a str, p2: & 'a str) -> & 'a str {
    if rand::random(){
        p1
    } else {
        p2
    }
}
