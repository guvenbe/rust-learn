// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

use thiserror::Error;


#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}
#[derive(Debug, Error)]
#[error("Invalid shirt color")]
struct ShirtColor(Color);
impl ShirtColor {
    pub fn new(color: Color) -> Result<Self, String> {
        match color{
            Color::Purple => Err("purple not allowed".to_owned()),
            other => Ok(Self(other))
        }
    }
}
#[derive(Debug)]
struct ShoeColor(Color);
impl ShoeColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}
#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    pub fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(color: ShirtColor){
    println!("shirt color = {:?}", color);
}
fn print_shoe_color(color: ShoeColor){
    println!("shoe color = {:?}", color)
}
fn print_pants_color(color: PantsColor){
    println!("pants color = {:?}", color)
}
fn main() {
    let shirt_color = ShirtColor::new(Color::Gray);
    let purple_shirt_color = ShirtColor::new(Color::Purple);
    let pants_color = PantsColor::new(Color::Blue);
    let shoe_color = ShoeColor::new(Color::White);
    
    print_shirt_color(shirt_color.expect("REASON"));
    print_pants_color(pants_color);
    print_shoe_color(shoe_color);
    match purple_shirt_color {
        Ok(shirt) => print_shirt_color(shirt),
        Err(e) => println!("Error: {}", e),
    }
}