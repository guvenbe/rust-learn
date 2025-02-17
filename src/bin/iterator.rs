fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_copy = numbers.clone();
    let mut plus_one: Vec<i32> = vec![];
    
    for num in &numbers{
        plus_one.push(num + 1);
    }
    let mut plus_one: Vec<_> = numbers_copy
        .iter()
        .map(|num| num + 1)
        .collect();
    
    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| *num % 2 != 0)
        .cloned()
        .collect();
    
    let numbers = vec![1,2,3,4,5];
    let find_me = numbers
        .iter()
        .find(|num| **num==3);
    let numbers = vec![1,2,3,4,5];
    let last =  numbers.iter()
        .last();
    let numbers = vec![1,2,3,4,5];
    let last =  numbers
        .iter().max();
    let numbers = vec![1,2,3,4,5];
    let last =  numbers
        .iter().min();
    let numbers = vec![1,2,3,4,5];
    let take: Vec<_> =  numbers
        .iter().take(3)
        .collect();
}