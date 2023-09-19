// Fix the code by addressing the TODO.

fn main() {
    let num_ref;
    let num = 23;
    {
        // TODO: shift below statement to appropriate location
        num_ref = &num;
    }
    println!("Reference points to {}", num_ref);
}