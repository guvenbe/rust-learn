fn main() {
    let tweet = String::from(
        "This is my tweet and it is very long"
    );

    let trimmed_tweet = &tweet[..20];
    println!("{trimmed_tweet}");

    //string literals are string slices

    let s = "hello world";
}