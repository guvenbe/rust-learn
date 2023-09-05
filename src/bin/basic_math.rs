fn main(){
    let mut result= sum (5, 5);
    display_result(result);

    result= substract (5, 2);
    display_result(result);

    result= multiply (5, 5);
    display_result(result);

    result= divide (20, 5);
    display_result(result);
}

fn sum (a: i32, b: i32) -> i32{
    a + b
}

fn substract (a: i32, b: i32) -> i32{
    a - b
}

fn multiply (a: i32, b: i32) -> i32{
    a * b
}

fn divide (a: i32, b: i32) -> i32{
    a / b
}

fn display_result(result : i32){
    println!("{:?}",result);
}