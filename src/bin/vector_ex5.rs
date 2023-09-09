struct Student {
    name: String,
    marks: u8,
    grade: char,
}
impl Student {
    fn new(name: &str, marks: u8) -> Self {
        Self {
            name: name.to_string(),
            marks,
            grade: 'X',
        }
    }
}
fn main() {
    let mut students = vec![
        Student::new("Harry", 75),
        Student::new("Hermoine", 99),
        Student::new("Ron", 60),
    ];

    for mut student in students.iter_mut() {
        student.grade = if student.marks > 80 {
            'A'
        }else if student.marks>60 {
            'B'
        }else {
            'C'
        }

    };
    for student in students {
        println!("{} got {}!", student.name, student.grade);
    }
}