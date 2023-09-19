
trait Double {
    fn double(&self) -> Self;
}

trait Printable {
    fn convert_to_str(self) -> String;
}

fn get_double_str<T>(input: T) -> String
where T: Double + Printable
{
    let doubled = input.double();
    doubled.convert_to_str()
}

//another way
fn get_double_str2(input: impl Double+Printable) -> String
{
    let doubled = input.double();
    doubled.convert_to_str()
}

impl Double for i32 {
    fn double(&self) -> Self {
        2 * self
    }
}

impl Printable for i32 {
    fn convert_to_str(self) -> String {
        format!("{self}")
    }
}

fn main() {
    let num = 22;
    let mut msg = format!("{num} doubled is ");
    msg.push_str(&get_double_str(num));
    println!("{msg}");
}