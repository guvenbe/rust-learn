#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}
pub fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = ll.next {
        v.add_up(10)
    }
    println!("ll: {:?}", ll);

    let mut v = Vec::with_capacity(100);
    v.push("hello".to_string());
    v.push("goodby".to_string());

    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("vlen ={}, capacity = {}", v.len(), v.capacity());
}
