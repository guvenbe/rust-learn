use std::{io::{self, Read}, fs::File};
use std::fmt::format;


fn main() {
    let contents = read_file("example.txt");
}

fn read_file(file_name: &str) -> Result<String, io::Error>{
    // let mut file =File::open(file_name)?; //? mark unwraps valid values or return erroneous values propagating them to the calling functions
    // let mut contents = String::new(); //heap allocated string
    // file.read_to_string(&mut contents)?;
    // Ok(contents) //if success returning contents

    // made more concise by inlining

    let mut contents = String::new();
    File::open(file_name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

struct  User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: User) -> Option<String>{
    let first_initial = user.firstname.chars().next()?;
    let last_initial = user.lastname.chars().next()?;
    Some(format!("{first_initial}.{last_initial}"))
}