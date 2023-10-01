use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    // let score_iter = scores.iter();
    // let first = scores.iter().next();
    //
    // let mut scores_iter_mut = scores.into_iter();

    for (team, score) in &scores{
        println!("{team} Got: {score} points")
    }

}