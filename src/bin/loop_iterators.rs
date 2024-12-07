pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}
fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n > 10 {
            break;
        }
        println!("Hello world! {}", n);
    }
    println!("All done LOOP");

    while n < 10 {
        n += 1;
        println!("Hello world! {}", n);
    }
    println!("All done WHILE");

    for i in 0..10 {
        println!("Hello world! {}", i);
    }
    println!("All done FOR");

    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };
    loop {
        match st.next() {
            Some(v) => println!("Ier Loop {}", v),
            None => break,
        }
    }

    let mut st = Stepper {
        curr: 3,
        step: 4,
        max: 20,
    };
    while let Some(n) = st.next() {
        println!("while Stepper {}", n);
    }

    let mut st = Stepper {
        curr: 3,
        step: 4,
        max: 20,
    };
    for i in st {
        println!("for Stepper {}", i);
    }
}
