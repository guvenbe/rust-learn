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

enum DispenserItem {
    Empty,
    Ammo(u8),
    Things(String, i32),
    Place { x: i32, y: i32 },
}

impl DispenserItem {
    fn display(&self) {}
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

    let mut my_var: Option<i32> = None;
    if let Some(x) = my_var {
        println!("value is {}", x);
    };

    match my_var {
        Some(x) => {
            println!("values is {}", x);
        }
        None => {
            println!("no value");
        }
    }
}
