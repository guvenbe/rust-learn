trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("parking car!");
    }
}
impl Park for Truck {
    fn park(&self) {
        println!("parking truck!");
    }
}
struct Truck {
    info: VehicleInfo,
}
impl Truck {
    fn unload(&self) {
        println!("unoading truck!")
    }
}

impl Paint for Truck {}

impl Paint for Car {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting house!")
    }
}

fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "civic".to_owned(),
            year: 2022,
        },
    };
    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);
    paint_yellow_park(&car);
    //paint_yellow_park(&house);
    //paint_yellow_park(&object);
}

// 3 ways to implement trait boundry
// We are saying type T that implement the paint trait

//1 Generic declaration
fn paint_red<T: Paint>(object: &T) {
    object.paint("Red".to_owned());
}

//2 Syntax sugar
fn paint_blue(object: &impl Paint) {
    object.paint("Blue".to_owned());
}

//3 multiple traits
fn paint_yellow_park<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("yellow".to_owned());
    object.park();
}

fn create_paintable_object() -> impl Paint {
    House {}
}
