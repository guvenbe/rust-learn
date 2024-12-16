struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &'a str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}
fn main() {
    let mut tweet = Tweet { content: "example" };
    tweet.replace_content("example2");
    println!("new content: {}", tweet.content);
}

//1. Each prarmeter that is a reference gets it's own lifetime parameter
//2.If there is exactly one input lifetime parameter . that lifetime is assigned
//  to all output lifetime parameters
//3.If there are multiple input lifetime parameters, but one of them is
//  is &self or mut &self the lifetime of self is assigend to all output
//  lifetime parameter

fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
    content
}
