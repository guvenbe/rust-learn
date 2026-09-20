1. Run the benchmarks with cargo bench.

2. Open the file target/criterion/sum\_even\_squares/report/index.html.

3. On this page you will see three benchmarks named loop, iter, and step. Each one has a mean execution time printed in milliseconds or microseconds.

4. To prove zero cost abstraction you compare these three mean times. If the numbers are very close to each other, for example loop = 100 ms, iter = 101 ms, step = 99 ms, that means they all run in the same time. In other words, the iterator versions are just as fast as the manual loop.

5. You can also look at the violin plot on that same page. It shows the distribution of run times. If the three plots overlap and have the same shape, it confirms they behave the same in performance.

6. If the numbers are too small to see a difference, increase the workload in benches/bench.rs by setting n = 10\_u64.pow(8) and run cargo bench again. Larger input makes the mean times easier to compare.

7. Criterion is already the main proof. Same mean times and overlapping plots demonstrate that the compiler removed the abstraction overhead.

8. If you want extra proof that there's no extra code, install cargo-bloat and run ir: cargo bloat -p zero_cost_iter --release.
If you see your three functions (loop, iter, step) and no extra core::iter::map or filter functions, it means the iterator code was inlined and removed. That is code size proof.
That is the complete way to prove it: Criterion report shows equal runtime, cargo-bloat shows no extra functions.

The output “0B filtered data size” means the compiler inlined your functions completely. They do not exist as separate symbols in the binary. This proves that the iterator code was optimized away, leaving no extra functions like `map` or `filter` at runtime.

How to prove this with cargo bloat in a simple way:

Run cargo bloat --release -p zero_cost_iter -n 20 without any filter.
If you don’t see core::iter::map, core::iter::filter, or similar functions, that shows they were inlined and eliminated.

No map/filter functions in the bloat output = optimizer removed them.

In other words: Criterion shows the same speed, and cargo-bloat shows nothing left for your iterator functions because the compiler inlined them. That is exactly what “zero-cost abstraction” means.

