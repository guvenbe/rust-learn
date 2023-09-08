struct  Product {
    name: String,
    category: category,
    price: f32,
    in_stock: bool,
}


enum ProductCategory{
    BOOKS,
    CLOTHING,
    ELECTRICS
}

fn main() {
    let category = ProductCategory::ELECTRICS;
    let product = Product {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true
    };
    
}