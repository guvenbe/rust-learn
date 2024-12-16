fn main() {
    let player1 = String::from("player_1");
    let player2 = String::from("player_2");
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    if rand::random() {
        p1
    } else {
        p2
    }
}
