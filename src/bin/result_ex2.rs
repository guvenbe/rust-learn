fn main() {
    let name = "Tom";

    if let Err(e) = greet(name){
        println!("Error : {e}");
    }
}

fn greet(name: &str) -> Result<(), String> {
    if name.len() > 0 {
        println!("Hello {name}");
        Ok(())
    }else {
        Err("empty name provided".to_string())
    }
}