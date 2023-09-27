trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct MyStruct {}

impl Iterator for MyStruct{
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

trait IntoIterator{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
fn main() {
    let mut m = MyStruct{};
    let item = m.next();
}