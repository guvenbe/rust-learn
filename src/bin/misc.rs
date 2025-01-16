use std::thread::current;

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    if let Ok(v) = a {
        println!("val = {}", v);
    }

    match &b {
        Ok(v) => println!("value:{v}"),
        Err(e) => println!("Error:{e}"),
    }
    println!("a = {:?}, b= {:?}", a, b);
    iter();
    
    let mut st2 = Stepper{curr :2, step:3, max:15};
    loop {
        match st2.next() {
            Some(v) => println!("loop step {v}"),
            None => break,
        }
    };
    
    let mut st = Stepper{curr :3, step:4, max:15};
    while let Some(n) = st.next(){
        println!("while; {n}")
    }
    
    for i in (Stepper{curr:5, step: 10, max: 50}) {
        println!("for loop step {}", i);
    }
}

pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr>=self.max{
            return None
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}
fn iter() {
    let mut n = 0;
    loop {
        n+=1;
        if n == 10 {
            break;
        }
        println!("Loop {n}")
    }
    n =0;
    while n<=10 {
        n += 1;
        println!(" while {n}");
    }
    
    for n in 1..10{
        println!("for: {n}")
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}
