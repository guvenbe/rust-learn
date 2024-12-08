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
        println!("parking car!")
    }
}

struct Truck {
    info: VehicleInfo,
}

impl Park for Truck {
    fn park(&self) {
        println!("parking trick!")
    }
}

impl Paint for Car {} //it has default implementation

impl Truck {
    fn unload(&self) {
        println!("inloading truck")
    }
}

impl Paint for Truck {} //it has default implementation

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house {}:", color);
    }
}
fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995,
        },
    };

    let house = House {};

    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);

    //These give error when passing i
    // paint_vehicle_red(&house);
    // paint_vehicle_red(&object);
}

// 3 way to set trait boundary
// we are saying ant type T that implements
// Paint trait

//1) at Type Generic declaration
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned())
}

//2 impl syntax sugar
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned())
}

//3 use where clause (If you need multiple trait bounds
fn paint_vehicle_red<T>(object: &T)
where
    T: Paint + Park,
{
    object.paint("red".to_owned())
}

fn create_paintable_object() -> impl Paint {
    House {}
}
