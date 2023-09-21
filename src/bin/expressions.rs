fn main() {
    let num = 3;

    let is_lt_5 = if 5 > num {
        true
    } else {
        false
    };

    //or like this
    let is_lt_5 = num > 5;

    let my_num = 3;
    let message = match my_num {
        1 => "hello",
        _ => "goodbye"
    };

    println!("{}", message);

    //Nested expression
    enum Menu {
        Burger,
        Fries,
        Drink,
    }

    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";

    let order_place = match  item {
        Menu::Drink => {
            if drink_type == "water"{
                true
            } else {
                false
            }
        }
        _ => true,
    };
}