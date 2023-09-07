fn main() {
    {
        let s1 = String::from("hello world");
    }
    //println!("{s1}")   //s1 does not have scope here

    //ownership
    let s2 = String::from("hello world");
    let s3 = s2; //ownership is transferred/moved to s3
    //println!("{s2}"); //will not work onwership moved to s3
    println!("{s3}");

    let s4 = String::from("hello world cloned");
    let s5 = s4.clone();
    println!("{s5}");

    //not true for some primitive types, these are cloned by default
    let x = 10;
    let y = x;
    println!("{x}")


}