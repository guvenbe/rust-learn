
fn main() {
    //let v: Vec<String> =Vec::new();
    let mut v = Vec::new();
    v.push(String::from("One"));
    v.push(String::from("Two"));
    v.push(String::from("Three"));

    // or use Vec! macro
    let v2 = vec![1, 2, 3];

    let s = &v[0]; //can panic
    // let s = v.remove(0);
    let s = v.get(0);
    if let Some(r) = s  {
        println!("{r}");
    }

    for s in &mut v{
        s.push_str("!")
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = vec![]; //empty vector
    for s in v  { //v is by value after for loop v is no longer valid
        v3.push(s)
    }


    //same

    for s in v.into_iter()  { //v is by value after for loop v is no longer valid
        v3.push(s)
    }
    //let i = v.get(0); //not valid since v is moved
}