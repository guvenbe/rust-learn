enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();

    assert_eq!(x, numbers.0);
    assert_eq!(y, numbers.1);
    assert_eq!(z, numbers.2);
    let (employee, access) = ("Jake", Access::Full);

    let coord = (2,3);
    println!("{:?}, {:?}", coord.0, coord.1);
}
