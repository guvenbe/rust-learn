struct ThreeDCoord {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);
    let (x, y) = coord;
    println!("{:?} {:?}", x, y);
    let (name, age) = ("Emma", 20);
    let ThreeDCoord = (1, 4, 5);
    println!("{:?}", ThreeDCoord);

    let (x, y) = coordiante();
    if y > 5 {
        println!("y is greater than 5")
    } else if y < 5 {
        println!("y is less than 5")
    } else {
        println!("y is equal to 5");
    }
}

fn coordiante() -> (i32, i32) {
    (1, 6)
}
