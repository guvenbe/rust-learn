fn main() {
    let names = vec!["Alice", "Bob", "Cindy"];
    let index = 2;
    if let Some(name) = names.get(index) {
        println!("{name} is present at index {index}");
    } else {
        println!("invalid index {index}");
    }
}