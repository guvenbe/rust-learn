trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}
#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        //Eat 20 percent of carrot
        self.percent_left *= 0.8;
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };

    let mut grapes = Grapes { amount_left: 200 };

    carrot.bite();
    println!("Take a bite {:?}", carrot);
    nible(&mut carrot);
    println!("Multiple bytes {:?}", carrot);
    nible(&mut grapes);
    println!("Multiple bytes {:?}", grapes);
}

fn nible<T: Bite>(t: &mut T) {
    t.bite();
    t.bite();
}
