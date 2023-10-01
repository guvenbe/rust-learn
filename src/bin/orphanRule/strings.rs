struct LineItem{
    name: String,
    count: i32,
}

fn main() {
    let receipt = vec![
        LineItem{
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem{
            name: String::from("fruit"),
            count: 3
        },
    ];
}