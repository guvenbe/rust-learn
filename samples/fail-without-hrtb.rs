//! ```cargo
//! [package]
//! edition = "2021"
//! ```

trait Visitor<'a> {
    fn visit(&mut self, s: &'a str);
}

#[derive(Default)]
struct Count(usize);

impl<'a> Visitor<'a> for Count {
    fn visit(&mut self, s: &'a str) {
        self.0 += s.len();
    }
}

// Without HRTB
fn run<'a, 'b, P>(p: &mut P, a: &'a str, b: &'b str)
where
    P: Visitor<'a>,
{
    p.visit(a); // ok
    p.visit(b); // error
}

fn main() {
    let mut c = Count::default();
    let s1 = String::from("hello");
    let s2 = String::from("world");
    run(&mut c, &s1, &s2);
}