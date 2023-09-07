fn main() {
    let text = String::from("Today is a very warm and sunny day.");
    let words = ["very", "arm", "say", "sun", "dew"];
    let mut pos;

    println!("Text: {text}");

    for word in words {
        pos = find_substr_pos(&text, word);
        if pos == text.len() {
            println!("{word} is not present in text");
        } else {
            println!("{word} present at index {pos}");
        }
    }
}

fn find_substr_pos(text: &str, substr: &str) -> usize {
    if text.len() < substr.len(){
        return text.len();
    }
    let len = substr.len();
    for start in 0..text.len() - len +1{
        let debug_str =&text[start..start + len];
        println!("{debug_str} -- {substr}");
        if substr == &text[start..start + len] {
            return start;
        }
    }
    text.len()
}