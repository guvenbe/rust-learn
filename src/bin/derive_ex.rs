#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    println!("employee = {:?}", me);
    print_employee(me);
    //This will work beacuse of derive clone a copy is being made
    print_employee(me);
    print_employee(me);
    print_employee(me);
}
