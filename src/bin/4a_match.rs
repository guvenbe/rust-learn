mod new_type_pattern_demo;

fn main(){
    let my_bool = true;

    match my_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}