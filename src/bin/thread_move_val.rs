use std::thread;
fn main(){
    let s = "Lets's get Rusty".to_owned();
    let handle  = thread::spawn(move ||{
        println!("{s}");
    });
    handle.join().unwrap();
}