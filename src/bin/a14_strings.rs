// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}

fn main() {
    let people = vec![
        Person {
            name: "Bob".to_owned(),
            age: 44,
            favorite_color: "Red".to_owned(),
        },
        Person {
            name: "Alice".to_owned(),
            age: 24,
            favorite_color: "Purple".to_owned(),
        },
        Person {
            name: "Jimmy".to_owned(),
            age: 9,
            favorite_color: "Blue".to_owned(),
        },
    ];

    for person in people{
        if person.age <= 10{
            println!("name: {:?}, age: {:?}, favorite color: {:?}",
                     person.name, person.age, person.favorite_color);
            print(&person.name);
            print(&person.favorite_color);

        }
    }
}

fn print (data: &str){
    println!("{:?}", data);
}