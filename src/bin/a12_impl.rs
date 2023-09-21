// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[derive(Debug)]
enum Color {
    Brown,
    Black,
    White,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Black => println!("Black"),
            Color::White => println!("White"),
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn new(length: f64, width: f64, height: f64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
    fn print(&self) {
        println!("Length: {:?}", self.length);
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}

fn main() {
    let box1 = ShippingBox::new(
        10.0,
        Color::Brown,
        Dimensions::new(
            2.0,
            4.0,
            3.0,
        ),
    );
    box1.print();
}