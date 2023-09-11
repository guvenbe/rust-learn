const MAX_PLAYERS: u8 = 10;
const NUMBER: i32 = 3;
static CASINO_NAME: &str = "Rusty Casino";
fn main(){
    let a = MAX_PLAYERS; //This will be inlined
    let  b  = MAX_PLAYERS;

    let c = CASINO_NAME; // occupy a location in memory
    let d = CASINO_NAME;

    let mut s1 = String::from("LGR");
    let r1 = &s1;
    let r2 = &mut s1;
    println!("{r1}");
}