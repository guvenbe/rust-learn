trait Sound{
    fn make_sound(&self);
}
trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Snake;
impl Move for Snake {
    fn move_to(&self, x: i32, y: i32) {
        println!("slither to {} {}", x, y)
    }
}
impl Sound for Snake{
    fn make_sound(&self){
        println!("SSSSSS!");
    }
}
struct Grasshopper;
impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("hop to {} {}", x, y)
    }
}
impl Sound for Grasshopper{
    fn make_sound(&self) {
        println!("Brrrrrt!")
    }
}
fn make_move<T: Move>(thing: T, x:i32, y:i32) {
    thing.move_to(x, y);
}

fn make_move_sound<T>(thing: T)
where T: Move + Sound {
    thing.make_sound();
    thing.move_to(1,1)
}
fn main() {
    let phython = Snake;
    make_move(phython, 1,1);
    let rattle_snake= Snake;
    make_move_sound(Snake)
}

