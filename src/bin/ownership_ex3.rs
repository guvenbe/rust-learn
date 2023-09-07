fn main() {
    let my_string = String::from("I love rust bootcamp 💕");
    let occurence_count = count_occurences(my_string.clone(), 'o');
    println!("The number of time 'o' appears in \"{my_string}\" = {occurence_count}")

}

fn count_occurences(text: String, letter: char) -> u32{
    let mut res = 0;
    for ch in text.chars() {
        if ch == letter {
            res += 1;
        }
    }
    res
}