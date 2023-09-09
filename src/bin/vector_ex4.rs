struct Student {
    name: String,
    marks: u8,
}

impl Student {
    fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
        }
    }
}

fn main() {
    let students = vec![
        Student::new("Harry", 75),
        Student::new("Harmoine", 99),
        Student::new("Ron", 60),
    ];

    let mut grades = Vec::new();
    for student in students.iter(){
        if student.marks > 80 {
            grades.push('A');
        } else if student.marks > 60 {
            grades.push('B');
        } else {
            grades.push('C');
        }
    }
    for i in 0..grades.len() {
        println!("{} got {}!", students[i].name, grades[i]);
    }
}
