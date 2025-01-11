fn main() {
    let mut s = "Let's get Rusty".to_owned();
    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;
    
    let address = 0x12344usize;
    let raw3 = address as *const String;
    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is {}", *raw1);
    }
    //immutable raw
    let num  = 123;
    let ptr = &num as *const i32;
    unsafe {
        println!("{} stored at {:p}" ,*ptr, ptr);
    }
    // print first 10 fibonacci numbers
    let (mut a, mut b) = (1, 0);
    let mut c = 0;
    let ptr_a = &mut a as *mut i32;
    let ptr_b = &mut b as *mut i32;
    let ptr_c = &mut c as *mut i32;
    unsafe {
        for _ in 0..10 {
            *ptr_c = *ptr_a + *ptr_b;
            println!("{}", *ptr_c);
            *ptr_a = *ptr_b;
            *ptr_b = *ptr_c;
        }
    }
}