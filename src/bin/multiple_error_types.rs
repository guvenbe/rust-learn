use std::{error, fs, io};
use std::num::ParseIntError;

fn main() {
    let i = parse_file2("example.txt");

    match i {
        Ok(i) => println!("{i}"),
        Err(e) =>{
            match e {
                ParseFileError::File => {}
                ParseFileError::Parse(e) => {}
            }
        }
    }
}

//Callers of the function won't the types of errors that can be passed, hance will not know how to handle them
//But it is simple
fn parse_file(filename: &str) ->Result<i32, Box<dyn error::Error>>{ //since we error is different type implements the error trait
    let s =fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)
}

enum ParseFileError{
    File,
    Parse(ParseIntError),
}

//callers can distinguish between the File and Parse error
fn parse_file2(filename: &str) ->Result<i32, ParseFileError>{
    let s =fs::read_to_string(filename).map_err(|e|ParseFileError::File)?;
    let i = s.parse().map_err(|e|ParseFileError::Parse(e))?;
    Ok(i)
}