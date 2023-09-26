fn main() {
    //This is a closure
    let ten = 10;
    let greater_than = |x: &i32| *x > ten;

    let less_than = |x: &i32| *x < 20;


    let result = are_both_true(greater_than, less_than, &15);
    println!("{result}");

    //passing function pointer
    let result = are_both_true_pointers(greater_than, less_then2, &15);
}

fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}

fn are_both_true_pointers<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where T: Fn(&V) -> bool, U: Fn(&V) -> bool {
    f1(item) && f2(item)
}

fn less_then2(x: &i32) -> bool {
    *x < 20
}