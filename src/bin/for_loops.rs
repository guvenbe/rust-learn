fn main() {
    println!("Hello World");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    for element in arr {
        println!("element {}", element);
    }

    for i in 1..10 {
        println!("i={}", i);
    }

    for i in 1..=5 {
        println!("izz={}", i);
    }

    let vec = vec![100, 200, 300, 400, 500];
    for val in vec {
        println!("val:{}", val);
    }

    let vec2 = vec![100, 200, 300, 400, 500];
    for (index, value) in vec2.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
    let mut vec3 = vec![100, 200, 300, 400, 500];
    for val in vec3.iter_mut() {
        *val *= 2;
    }
    println!("Modified Vector {:?}", vec3);
}
