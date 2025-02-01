fn main(){
    let mut s1 = String::from("LGR");
    let r1 = &s1;
    let r2 = &mut s1;
    println!("{r2}")
}