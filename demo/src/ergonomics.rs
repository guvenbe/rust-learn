//! Compiler ergonomics: doctests that show common mistakes and the "fixed" versions.

//! Borrow checker catching a common mistake in threads.
//!
//! ```compile_fail
//! use std::thread;
//! fn main() {
//!     let data = vec![1,2,3];
//!     // Error: `data` does not live long enough for the spawned thread.
//!     thread::spawn(|| {
//!         println!("{:?}", data);
//!     });
//! }
//! ```
//!
//! The compiler suggests capturing by move (or ensuring 'static). Fixed version:
//!
//! ```rust
//! use std::thread;
//! fn main() {
//!     let data = vec![1,2,3];
//!     let handle = thread::spawn(move || {
//!         println!("{:?}", data);
//!     });
//!     handle.join().unwrap();
//! }
//! ```

//! Another classic: mutable aliasing prevented at compile time.
//!
//! ```compile_fail
//! let mut v = vec![1,2,3];
//! let r1 = &v;
//! let r2 = &mut v; // cannot borrow `v` as mutable because it is also borrowed as immutable
//! println!("{:?}", r1);
//! *r2 = vec![4,5,6];
//! ```
//!
//! Corrected by scoping borrows so they don't overlap:
//!
//! ```rust
//! let mut v = vec![1,2,3];
//! {
//!     let r1 = &v;
//!     println!("{:?}", r1);
//! } // r1 out of scope here
//! let r2 = &mut v;
//! r2.push(4);
//! ```
pub fn marker() {}
