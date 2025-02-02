#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student1 = Student {
        name: "Bora".to_owned(),
        locker: None,
    };
    println!("student1: {:?}", student1);
    
    println!("name: {:?}", student1.name);
    
    match student1.locker {
        Some(num) => println!("locker num: {}", num),
        None => println!("no locker assigned") 
    }
    
    
}