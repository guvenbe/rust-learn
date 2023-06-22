struct Grocery_Item {
    stock: i32,
    price: f64,
}
fn main() {
    let cereal = Grocery_Item {
        stock: 10,
        price: 2.99,
    };

    println!("Stock {:?}", cereal.stock);
    println!("price {:?}", cereal.price);
}
