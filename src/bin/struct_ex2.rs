struct Shopitem {
    name: String,
    quantity: u32,
    in_stock: bool,
}

fn main() {
    let mut item = Shopitem {
        name : String::from("Socks"),
        quantity:200,
        in_stock: true
    };
    if item.quantity -=50 {
        item.in_stock = false;
    }
    println!("{} is in stock: {}", item.name, item.in_stock);
}