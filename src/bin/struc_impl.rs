#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}
impl Product {
    fn new(name: String, price: f32) -> Product {
        Product {
            name,
            price,
            in_stock: true,
        }
    }
    fn get_default_sales_tax() -> f32 {
        0.1
    }
    fn calculate_sales_tax(&self) -> f32 {
        self.price * Product::get_default_sales_tax()
    }
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }
    fn buy(self) -> i32 {
        let name = self.name;
        println!("{} was bought", name);
        123
    }
}

fn main() {
    let mut book = Product {
        name: String::from("Book"),
        price: 28.85,
        in_stock: true,
    };
    let sales_tax: f32 = book.calculate_sales_tax();
    println!("{}", sales_tax);
    book.set_price(1.00);

    book.buy();
    let book2 = Product::new(String::from("Book2"), 30.0);
    let sales_tax = book2.calculate_sales_tax();
    println!("{}", sales_tax)
}
