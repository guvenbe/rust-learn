use zero_cost_iter::{sum_even_squares_iter, sum_even_squares_loop, sum_even_squares_step};

fn main() {
    let n = 10_000_000u64;
    let a = sum_even_squares_loop(n);
    let b = sum_even_squares_iter(n);
    let c = sum_even_squares_step(n);
    println!("loop   : {a}");
    println!("iter   : {b}");
    println!("step   : {c}");
}
