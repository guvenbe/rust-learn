struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl <T> BrowserCommand <T>{
    fn new(name: String, payload: T) -> Self {
        BrowserCommand {
            name,
            payload
        }
    }
    fn get_payload(&self) ->&T{
        &self.payload
    }
}

impl BrowserCommand<String>{
    fn print_payload(&self){
        println!("{}", self.payload)
    }
}
fn main() {
    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "https://www.letsgetrusty.com".to_owned(),
    );
    let cmd2 = BrowserCommand::new(
        "zoom".to_owned(),
        200,
    );

    cmd1.print_payload(); //only work for cmd1 since payload is a string
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serailize_payload(p1); //rust does monomorphization
    serailize_payload(p2);

}

fn serailize_payload<T>(payload: T) -> String{
    //convert payload to JSON string
    "placeholder".to_owned()
}