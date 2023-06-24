use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(4);
    v.push(6);
    let x = v.pop();
    println!("{}", v[1]);

    let mut v2 = vec![2, ,4, 6, 8];

    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(5, true);
    h.insert(6, false);
    h.insert(7, true);
    let have_five = h.remove(&5).unwrap();
}
