
fn main() {
    let player1 = String::from("player1");
    {
        let player2 = String::from("player2");

        let result = first_turn(player1.as_str(), player2.as_str());

        println!("player going first is {}", result);
    }
}

fn first_turn<'a>(p1: &str, p2: &str) -> & 'static str {
    //static string slice. lives the duration of the program
    let s: &'static str = "Let's Get Rusty";
    s
}
