struct Tweet<'a>{
    content: & 'a str,
}

impl <'a>Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content =self.content;
        self.content = content;
        old_content
    }
}
fn main() {
    let mut tweet = Tweet {
        content: "Example",
    };
    let old_content  = tweet.replace_content("replaced example");
    println!("{old_content}");
    println!("{}", tweet.content);
}

// 1. Each parameter that is a reference gets it's own lifetime parameter
// 2. If there is exactly one input lifetime parameter, thatlifetime is assigned to all output
//    lifetime parameters
// 3. if thereare multiple input lifetime parameters, but one them is &self or &mut self the lifetime
//    the lifetime of self is assigned to all output lifetime parameters,
fn take_and_return_content<'a, 'b>(content: &'a str,  content2: &'a str ) -> &'a str{
    content
}