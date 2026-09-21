//! Small executable that wires everything together and prints quick stats.

use std::sync::Arc;
use std::time::Duration;

use systems_mindset_demo::{
    concurrency::{Pipeline, parse_heavy_payload, safe_concurrent_counter},
    store::TelemetryStore,
    zero_cost::{count_even_numbers, count_matching},
    lifetimes::longest_str,
};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // 1) Fearless concurrency: thread-safe counter
    let total = safe_concurrent_counter();
    println!("[threads] safe_concurrent_counter -> {total}");

    // 2) Service wiring: async pipeline producing and storing telemetry
    let store = Arc::new(TelemetryStore::new());
    let pipeline = Pipeline::new(store.clone(), /*devices*/ 64, /*rate*/ Duration::from_millis(20));
    pipeline.run_for(Duration::from_secs(2)).await;

    // 3) Zero-cost style query over real data
    if let Some(avg) = store.iter_last_n_average("device-0007", 16) {
        println!("[iterators] last-16 average for device-0007 -> {avg:.3}");
    } else {
        println!("[iterators] no data for device-0007");
    }

    // 4) Lifetimes: simple demo
    let a = "alpha";
    let b = "alphabet";
    let longer = longest_str(a, b);
    println!("[lifetimes] longest_str -> {longer}");

    // 5) Higher-order / monomorphized generic utility
    let evens = count_even_numbers(&[1,2,3,4,5,6,7,8,9,10]);
    let odds = count_matching(1..=10, |n| n % 2 == 1);
    println!("[zero-cost] evens={evens}, odds={odds}");

    // 6) CPU-bound isolation with spawn_blocking
    let digits = parse_heavy_payload("payload-42-with-1337-digits-007".into()).await;
    println!("[spawn_blocking] digit count = {digits}");

    println!("Done.");
}
