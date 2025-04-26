trait NumericExt {
    fn double(&self) -> Self;
    fn is_even(&self) -> bool;
}
impl NumericExt for i32 {
    fn double(&self) -> Self {
        *self * 2
    }

    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}
fn main() {
    let number = 4;
    let doubled = number.double();
    let is_even = number.is_even();
    println!("doubled:{doubled}, iseven:{is_even}")
}
