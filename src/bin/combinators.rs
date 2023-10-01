#[derive(Debug)]
struct Student{
    name: String,
    gpa: f32
}
fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    // let good_students: Vec<Student> = students.iter()
    //     .map(|s| {
    //         let mut s = s.split(' ');
    //         let name = s.next()?.to_owned();
    //         let gpa = s.next()?.parse::<f32>().ok()?;
    //
    //         Some(Student { name, gpa })
    //     })
    //     .flatten()
    //     .filter(|s| s.gpa >= 3.5)
    //     .collect();
    //
    // for s in good_students {
    //     println!("{:?}", s);
    // }

    let good_students: Vec<Student> = students.iter()
        .flat_map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student { name, gpa })
        })
        .collect();

    for s in good_students {
        println!("{:?}", s);
    }
    println!();
    println!();

    // ----------------------------------

    let mut good_students = vec![];

    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>();

            if let Ok(gpa) = gpa {
                if gpa >= 3.5 {
                    good_students.push(Student { name, gpa });
                }
            }
        }
    }

    for s in good_students {
        println!("{:?}", s);
    }
}