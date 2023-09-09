// Complete the function to make the program execute successfully.

fn remove_if_odd(nums: &mut Vec<i32>, index: usize) {
    if index > nums.len()  {
        println!("Index out of bounds");
        return;
    }

    if  nums[index] %2 == 1 {
        nums.remove(index);
    }
}

fn main() {
    let mut nums = vec![1, 2, 6, 9];
    let nums_ref = &mut nums;
    remove_if_odd(nums_ref, 0);
    remove_if_odd(nums_ref, 1);
    remove_if_odd(nums_ref, nums_ref.len() - 1);
    assert_eq!(nums.len(), 2);
}
