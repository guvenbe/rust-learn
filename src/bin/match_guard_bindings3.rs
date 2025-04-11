#[derive(Debug,Eq, PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}
fn main() {
    let stage = 5;
    let diff = Difficulty::Normal;
    
    match stage {
        s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage: {}", s),
        s if diff == Difficulty::Normal => println!("normal difficulty stage {}", s),
        s @ 10 | s @ 15 => println!("stage 10 or 15"),
        s => println!("stage {}", stage),
    }
}
