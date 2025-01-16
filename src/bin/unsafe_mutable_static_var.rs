static mut COUNTER: i32 = 0;

fn increment_counter(){
    unsafe {
        COUNTER +=1; }
}
fn main() {
    for _ in 0..10{
        increment_counter();
    }
}