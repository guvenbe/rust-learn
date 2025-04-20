struct UpperCase(String);

impl From<String> for UpperCase{
    fn from(data: String) -> Self{
        UpperCase(data.to_uppercase())
    }
}
impl From<&str> for UpperCase{
    fn from(data: &str) -> Self{
        UpperCase(data.to_uppercase())
    }
}
fn main() {
    let upper = UpperCase::from("lowercase");
    let upper: UpperCase = "lowercase".into();
    
}