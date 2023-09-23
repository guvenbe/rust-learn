use std::fs::File;

fn main() {
    get_file1();
}

fn get_file(){
    //If you are  prototyping. If Ok is returned unwrap will return the value else it will panic
    let file = File::open("example.txt").unwrap();
    //Other examples
    let file = File::open("example.txt").expect("Failed open file");

}
fn get_file1() {
    let file = File::open("example.txt");
    //shadow file
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Failed to open file: {:?}", error)
    };
}

fn get_user_id(username: &str) -> Result<i32, String> {
    if username.is_empty() {
        Err("Username can not be empty".to_owned())
    } else {
        Ok(1)
    }
}