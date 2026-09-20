use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use zero_cost_iter::{sum_even_squares_iter, sum_even_squares_loop, sum_even_squares_step};

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_even_squares");
    for &n in &[10_u64.pow(5), 10_u64.pow(6), 10_u64.pow(7)] {
        group.throughput(Throughput::Elements(n));
        group.bench_with_input(BenchmarkId::new("loop", n), &n, |b, &n| {
            b.iter_batched(|| n, |n| sum_even_squares_loop(n), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("iter", n), &n, |b, &n| {
            b.iter_batched(|| n, |n| sum_even_squares_iter(n), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("step", n), &n, |b, &n| {
            b.iter_batched(|| n, |n| sum_even_squares_step(n), BatchSize::SmallInput)
        });
    }
    group.finish();
}

criterion_group!(benches, bench_sum);
criterion_main!(benches);
