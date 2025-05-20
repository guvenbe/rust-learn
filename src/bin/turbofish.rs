use tokio_stream::iter;

fn main() {
    let numbers = vec![1,2,3];
    
    let odd = numbers
        .iter()
        .filter(|n| **n % 2 == 1)
        .collect::<Vec<_>>();
}