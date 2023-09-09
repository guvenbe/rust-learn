// Complete the function to make the program execute successfully.

fn append(nums: &mut Vec<i32>, num: i32) {
    nums.push(num);
}

fn main() {
    let mut nums = vec![1, 2, 5, 6];
    append(&mut nums, 8);
    append(&mut nums, 3);
    assert_eq!(nums.len(), 6);
}