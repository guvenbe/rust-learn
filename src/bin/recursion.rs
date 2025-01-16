use num_bigint::BigInt;
use num_traits::{One, Zero};

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
//tail recusion
fn tfactorial(n: BigInt, acc: BigInt) -> BigInt {
    if n <= One::one (){
        return acc;
    }
    return tfactorial(n.clone() - 1u32, n * acc);
}
fn main() {
    let mut x = factorial(5);
    println!("factorial(5) = {x}");
    let mut x = tfactorial(100u32.into(),BigInt::one());
    println!("tfact={x}")
}
