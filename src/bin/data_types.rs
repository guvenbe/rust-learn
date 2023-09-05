fn main() {
    let b1: bool = true;

    //unsigned integers
    let u1: u8 = 1;
    let u1: u16 = 1;
    let u1: u32 = 1;
    let u1: u64 = 1;
    let u1: u128 = 1;

    //signed integers
    let i1: i8 = 1;
    let i1: i16 = 1;
    let i1: i32 = 1;
    let i1: i64 = 1;
    let i1: i128 = 1;

    //float
    let f1: f32 = 1.0;
    let f2: f64 = 2.0;

    //characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    //arrays
    let a1: [i32; 5] = [1,2,3,4,5]; //same type
    let i50 =a[4];

    //tuples

    let t1: (i32, i32, i32) =  (1,2,3);
    let t2: (i32, f64, &str) = (5, 5.0, "5");

    let s1 = t.2;
    //destructure
    let (i60, f3, s9 ) = t2;

    let unit: () = ();

    //type aliasing
    type age = u8;
    let age1 = age;
}