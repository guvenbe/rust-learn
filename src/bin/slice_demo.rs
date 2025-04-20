fn main() {
    let chars = vec!['A', 'B', 'C', 'D'];
    // let chars = vec!['A'];
    match chars.as_slice() {
        [first,..,last]=> println!("{first},{last}"),
        [single] => println!("{single}"),
        [] => (),
    }
    let chars = vec!['A', 'B', 'C', 'D'];
    match chars.as_slice() {
        [one,two, ..]=> println!("{one}, {two}"),
        [.., last] => println!("{last}"),
        [] => (),
    }
    let nums = vec![1,8,9];
    match nums.as_slice() {
        [first @ 1..=3, rest @ ..] => println!("{:?}, {:?} ", first, rest),
        [single] if single == &5 || single == &6 => (),
        [a,b] => (),
        [..] => (),
    }
}