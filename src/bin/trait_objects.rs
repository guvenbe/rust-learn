

//Any type implementing Vehicle trait must also implement the Paint trait
trait Vehicle: Paint {
    fn park(&self);
    //Associated function
    fn get_defual_color() -> String{
        "black".to_owned()
    }
}

trait Paint{
    fn paint(&self, color: String){
        println!("painting object: {}", color);
    }
}
struct VehicleInfo{
    make: String,
    model: String,
    year: u16,
}
struct Car {
    info: VehicleInfo,
}


impl Vehicle for Car{
    fn park(&self) {
        println!("parking car!")
    }
}

struct Truck {
    info: VehicleInfo,
}

impl Vehicle for Truck {
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

struct House{}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house:");
    }
}
fn main() {
    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };

    let house = House {};

    let object = create_paintable_object(true);

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

    paint_vehicle_red(&car);

    //These give error when passing i
    // paint_vehicle_red(&house);
    // paint_vehicle_red(&object);

}

// 3 way to set trait boundary
// we are saying ant type T that implements
// Paint trait

//1) at Type Generic declaration
fn paint_red(object: &dyn Paint){
    object.paint("red".to_owned())

}

//2 impl syntax sugar
fn paint_red2(object: & impl Paint){
    object.paint("red".to_owned())

}

//3 use where clause (If you need multiple trait bounds
fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
    object.paint("red".to_owned())

}


//Box<dyn ..) is needed when multiple trait types can be returned, Hence returning a trait object
fn create_paintable_object(vehicle: bool) -> Box<dyn Paint>{
    if vehicle {
        Box::new(Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
            }
        })
    } else {
        Box::new(House{})
    }

}

