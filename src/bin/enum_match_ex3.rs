enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(f64),
    Promo(PromoDiscount),
    Custom(String),
}

struct Ticket {
    event: String,
    price: f32,
}

fn main() {
    Discount::Flat(100.0);
    Discount::Promo(PromoDiscount::Holiday("Christmas".to_owned()));

    let n = 3;
    match n {
        3 => println!("there"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(2.0);

    match flat {
        Discount::Flat(2.0) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat distcount of {:?}", amount),
        _ => (),
    }
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50.0,
    };

    match concert {
        Ticket { price: 50.0, event } => println!("event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
