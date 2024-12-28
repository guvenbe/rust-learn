fn main() {
    let res =gcd(8,12);
    println!("{}", res);
}

fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // exchange the value of m, n if m<n
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n; //Update m to the remainder  of m/n m become zero
    }
    n
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(8,4), 4);
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(2*3*5*11*17,3*7*11*13*19), 3*11);
}