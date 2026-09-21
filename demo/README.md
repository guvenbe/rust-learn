# Systems Mindset Demo (Rust)

This near real-world example implements a small telemetry ingestion & query
service to demonstrate:

- Zero-cost abstractions (iterators, generics/monomorphization)
- Fearless concurrency (threads + async pipelines)
- Compiler ergonomics (doctests that show borrow checker help)
- Lifetimes (borrowing & views)

## How to run

cargo run
cargo test   # runs doctests too

## Text → Code Map

1) Zero-Cost Abstractions
   - src/zero_cost.rs (iterators vs loops, generic map/filter/count)
   - src/store.rs::iter_last_n_average (iterator-based aggregation)

2) Fearless Concurrency
   - src/concurrency.rs::safe_concurrent_counter (threads, Arc<Mutex<_>>)
   - src/concurrency.rs::Pipeline (Tokio async, mpsc, backpressure)
   - src/concurrency.rs::parse_heavy_payload (spawn_blocking)

3) Compiler Ergonomics
   - src/ergonomics.rs doctests with `compile_fail` and fixed alternatives

4) Lifetimes
   - src/lifetimes.rs::longest_str (classic reference-tying example)
   - src/lifetimes.rs::TelemetryView (borrowed "view" API)

## Notes

- The store uses DashMap for concurrent writes. A ring buffer would be a straightforward extension if history bounding is required.
- The async pipeline simulates 64 devices emitting telemetry; tweak rates and counts in main.rs.
- Release profile enables thin LTO and fewer codegen units to reflect perf-conscious builds.
