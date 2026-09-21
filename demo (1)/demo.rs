//! ```cargo
//! [package]
//! name = "advanced_borrowing"
//! version = "0.1.0"
//! edition = "2021"
//!
//! [dependencies]
//! ```

/// Demonstrates advanced borrowing scenarios in Rust.
fn main() {
    println!("=== Advanced Borrowing Scenarios ===\n");

    // --- 1. Complex Mutable and Immutable Reference Interactions ---
    println!("1. Complex Mutable/Immutable Reference Interactions");

    let mut data = vec![1, 2, 3, 4, 5];
    println!("   Initial data: {:?}", data);

    // You can have multiple immutable references simultaneously.
    let r1 = &data[0];
    let r2 = &data[1];
    println!("   Reading via immutable refs: {}, {}", r1, r2);

    // However, you cannot have a mutable reference while immutable ones exist.
    // Uncommenting the next line would cause a compile error:
    // let r3 = &mut data[2]; // Error: cannot borrow `data` as mutable because it is also borrowed as immutable
    // println!("   {}", r3);

    // The immutable references `r1` and `r2` must go out of scope
    // before we can create a mutable one.
    drop(r1); // Explicitly drop r1
    drop(r2); // Explicitly drop r2
    // Now it's okay to create a mutable reference.
    let r3 = &mut data[2];
    *r3 += 10;
    println!("   Modified data[2] via mutable ref: {:?}", data);

    // Once the mutable reference `r3` goes out of scope (end of its lifetime),
    // immutable references can be created again.
    // The scope of `r3` ends after its last use, which is the `*r3 += 10` line.
    // So we can create new immutable refs here.
    let r4 = &data[0];
    let r5 = &data[1];
    println!("   Reading again via immutable refs: {}, {}\n", r4, r5);

    // --- 2. Non-Lexical Lifetimes (NLL) in Practice ---
    println!("2. Non-Lexical Lifetimes (NLL) in Practice");

    let mut numbers = vec![10, 20, 30];
    println!("   Initial numbers: {:?}", numbers);

    // Before NLL, this pattern was often problematic.
    // Get an immutable reference.
    let first = &numbers[0];
    println!("   First number (immutable): {}", first);

    // In older Rust (pre-NLL), the compiler would conservatively assume
    // that `first` lived until the end of the current lexical scope,
    // preventing the mutable borrow below even though `first` is no longer used.
    // With NLL, the lifetime of `first` ends after its last use (`println!` above).
    // Therefore, this mutable borrow is now allowed.
    numbers.push(40); // This mutable borrow is allowed because `first`'s lifetime has ended.
    println!("   After push: {:?}", numbers);

    // Get another immutable reference.
    let second = &numbers[1];
    println!("   Second number (immutable): {}", second);

    // Similarly, `second`'s lifetime ends after the `println!`.
    // We can get a mutable reference again.
    numbers[1] += 5;
    println!("   After modifying index 1: {:?}", numbers);

    // NLL allows more flexible borrowing by tracking when references are *actually* used,
    // not just where they are declared.
    println!("   NLL allows mutable borrows after immutable refs are last used.\n");

    // --- 3. Navigating the Borrow Checker in Challenging Cases ---
    println!("3. Navigating the Borrow Checker: Splitting Borrows");

    let mut complex_data = vec![100, 200, 300, 400];
    println!("   Complex  {:?}", complex_data);

    // Challenge: Modify two different elements of a vector.
    // Naive approach that fails:
    // let first_elem = &mut complex_data[0];
    // let second_elem = &mut complex_data[1]; // Error: second mutable borrow
    // *first_elem += 1;
    // *second_elem += 1;

    // Solution 1: Use separate scopes to limit borrow lifetimes.
    {
        let first_elem = &mut complex_data[0];
        *first_elem += 1; // Lifetime of `first_elem` ends here.
    } // `first_elem` is dropped.
    {
        let second_elem = &mut complex_data[1];
        *second_elem += 1; // This is now allowed.
    } // `second_elem` is dropped.
    println!("   After separate scopes modification: {:?}", complex_data);

    // Solution 2: Use methods that split the borrow.
    // `split_at_mut` allows borrowing two non-overlapping parts of a slice.
    let (left, right) = complex_data.split_at_mut(1);
    left[0] += 10;  // Modify the first part
    right[0] += 10; // Modify the first element of the second part (index 2 of original)
    println!("   After split_at_mut modification: {:?}", complex_data);

    // Solution 3: Use indexing if you know the indices are different.
    // This works because the compiler can see the borrows are to distinct elements.
    // However, this pattern is less common and can be tricky with dynamic indices.
    // Generally, `split_at_mut` or separate scopes are preferred.
    // let elem_0 = &mut complex_data[0];
    // let elem_3 = &mut complex_data[3]; // This *might* work if compiler can prove disjointness
    // *elem_0 += 5;
    // *elem_3 += 5;
    // println!("   After direct distinct index modification: {:?}", complex_data);

    println!("\nBorrow checker challenges can often be solved by:");
    println!(" - Limiting the scope/lifetime of references.");
    println!(" - Using methods like `split_at_mut` for safe disjoint borrowing.");
    println!(" - Restructuring code to avoid conflicting borrows.");

    println!("\nAdvanced borrowing requires understanding:");
    println!(" - That mutable and immutable references cannot coexist.");
    println!(" - That Non-Lexical Lifetimes track actual usage, not just declaration scope.");
    println!(" - How to leverage Rust's APIs to achieve safe, complex borrowing patterns.");
}
