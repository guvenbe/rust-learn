// Complete the code by addressing the TODOs.

use std::rc::Rc;

struct Employee {
    name: String,
    id: u32,
}

impl Employee {
    fn new(name: &str, id: u32) -> Self {
        Employee {
            name: name.to_string(),
            id,
        }
    }
    fn print_details(&self) {
        println!("Name: {}, ID: {}", self.name, self.id);
    }
}

fn main() {
    let emp1 = Box::new(Employee::new("Alice", 1234));
    emp1.print_details();
    let emp2 = Box::new(emp1);
    emp2.print_details();
    let emp3 = Rc::new(emp2);
    emp3.print_details();
}