use std::collections::HashMap;

fn main() {
   let mut stock: HashMap<String, i16> = HashMap::new();
   stock.insert("Chairs".to_owned(), 5);
   stock.insert("Beds".to_owned(), 3);
   stock.insert("Tables".to_owned(), 2);
   stock.insert("Couches".to_owned(), 0);
   
   println!("Total # items:{:?}", stock.len());
   
   for (item, quantity) in stock{
      match quantity {
         0 => println!("{:?} out of stock", item),
         _ => println!("{:?}:{:?}",item, quantity),
      }
   }
}