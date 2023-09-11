use std::ops::Add;
struct Pair<T>(T, T);

impl<T> Pair <T>
where T: Add<Output = T> + Copy
{
    fn add(&self) -> T{
        self.0 + self.1
    }
}

fn main() {
    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}