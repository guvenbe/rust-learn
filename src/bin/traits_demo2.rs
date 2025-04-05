trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("slither to {} {}", x, y)
    }
}
struct Grasshopper;
impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("hop to {} {}", x, y)
    }
}

fn make_move(thing: impl Move, x: i32, y: i32) {
    thing.move_to(x, y);
}
fn main() {
    let phython = Snake;
    make_move(phython, 1,1);
}
