trait Speak {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn let_it_speak(animal: &impl Speak) {
    animal.speak();
}
fn main() {
    let dog = Dog;
    let cat = Cat;
    let_it_speak(&dog);
    let_it_speak(&cat);
    let_it_speak(&cat);
    let_it_speak(&);
}
