// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavor {
    Coffe,
    Cola,
    Orange
}

struct Drink {
    flavor: Flavor,
    ounces: f32,
}
fn main() {
    let drink = Drink{
        flavor : Flavor::Cola,
        ounces: 3.25
    };

    print_drink(drink);
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Coffe => println!("It's {:?}", drink.flavor),
        Flavor::Cola => println!("It's {:?} ", drink.flavor),
        Flavor::Orange => println!("It's {:?}", drink.flavor)
    };
    println!("oz: {:?}", drink.ounces)
}