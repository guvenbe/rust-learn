fn main(){
    let my_name = "Bora";

    match my_name {
        "Bora" => println!("thats is my name"),
        "Bob" => println!("hello Bob"),
        "Alice" => println!("hello Allice"),
        _  => println!("Hello nice to meet you")
    }
}