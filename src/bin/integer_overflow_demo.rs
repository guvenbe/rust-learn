use std::arch::aarch64::uint32x2_t;
use num_traits::ops::overflowing::OverflowingSub;

fn main() {

    // None
    let n: Option<u32> = 0u32.checked_sub(1);
    // None
    let n: Option<u32> = u32::MAX.checked_add(1);
    // Some(10)
    let n: Option<u32> = 9_u32.checked_add(1);
    
    // (4294967295, true)
    let n = 0u32.overflowing_sub(1);
    println!("{:?}", &n);
    let n = 9u32.overflowing_add(1);
    println!("{:?}", &n);
    
    //0
    let n = 0_u32.overflowing_sub(9001);
    println!("{:?}", &n);
    
    //4294967295
    let n = u32::MAX.saturating_add(u32::MAX);
    println!("{:?}", &n);
    
    
    // 4294967295
    let n = 1_u32.wrapping_sub(2);
    // 0
    let n = u32::MAX.wrapping_add(1)
    
    
}