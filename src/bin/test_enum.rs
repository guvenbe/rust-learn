enum Direction {
    Left,
    Right,
}
enum Color {
    Red,
    White,
    Blue,
    Green,
}
fn display_color(color: Color) {
    match color {
        Color::Red => println!("{}", "Red"),
        Color::White => println!("{}", "White"),
        Color::Blue => println!("{}", "Blue"),
        Color::Green => println!("{}", "Green"),
    }
}
fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }

    let color = Color::Red;
    display_color(color);
}
