#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fav_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

fn main() {
    println!("Hello, world!");
    for i in 1..10 {
        println!("hello {}", i);
    }
    println!("hello");

    let p = Person {
        name: "Bora".to_string(),
        age: 60,
        children: 2,
        fav_color: Color::Green,
    };

    let c = Color::Red(", my favorite color".to_string());

    match c {
        Color::Red(s) => println!("It's red{}", s),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    }
    //println!("Hello people, from {:?}", p.print(););
    println!("p={:?}", p)
}
