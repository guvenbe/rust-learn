use std::arch::asm;
fn add(x: u64, y: u64) -> u64 {
    let result: u64;
    unsafe {
        asm!(
        "add {0},{1},{2}",
        out(reg) result,
        in(reg)x,
        in(reg)y
        );
    }
    result
}
fn main() {
    unsafe {
        let res = add(4000000, 100000000000clang);
        println!("{}", res);
    }
}
