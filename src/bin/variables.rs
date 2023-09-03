fn main(){
    // creation
    let a: i32 = 5;
    println!("a is : {}", a);
    let i = 10;
    println!("i= {i}");

    //mutability

    //Does not work
    // let b = 9;
    // b =10;

    let mut b =10;
    b = 11;
    println!("b is : {b}");

    //shadowing , creating separate variables
    let c :i32 = 89;
    let c :i32 = 90;

    println!("c:{c}");

    //scope

    let d: i32 = 30;
    {
        let d : i32 = 50;
        println!("inner d:{d}")
    }

    println!("d is:{d}")

}