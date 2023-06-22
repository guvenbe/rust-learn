enum Flavor {
    Vanilla,
    Choclate,
    Strawberry,
}
struct Drink {
    flavor: Flavor,
    fluid_ounce: f64,
}

fn print_drink(drink: Drink) {
    println!("drink size {:?}", drink.fluid_ounce);
    match drink.flavor {
        Flavor::Vanilla => println!("Vanilla"),
        Flavor::Choclate => println!("Choclate"),
        Flavor::Strawberry => println!("Strawberry"),
    }
}
fn main() {
    let my_drink = Drink {
        flavor: Flavor::Vanilla,
        fluid_ounce: 16.2,
    };

    print_drink(my_drink);
}
