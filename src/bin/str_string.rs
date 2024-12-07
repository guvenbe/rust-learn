fn main() {
    let mut s = " hello ".to_string();
    println!("{s}");
    let p = s.trim();
    let p = s.to_string();
    println!("p=='{}'", p);
    s.push_str("goodbye");

    let fstr = " help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = {}", ffstr);

    println!("chosen = {}", choose_str(1));
}

fn string_find_f(s: &str) -> &str {
    let n = 0;

    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
