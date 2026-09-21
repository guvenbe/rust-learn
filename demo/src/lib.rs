//! Systems Mindset Demo Library
//! This crate demonstrates:
//! - Zero-cost abstractions (iterators, generics/monomorphization)
//! - Fearless concurrency (threads + async pipelines)
//! - Compiler ergonomics (helpful errors via compile_fail doctests)
//! - Lifetimes (borrowing views over data)

pub mod domain;
pub mod store;
pub mod zero_cost;
pub mod concurrency;
pub mod ergonomics;
pub mod lifetimes;
