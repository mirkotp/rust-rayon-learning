use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon::prelude::*;

// Something that can run for a while
fn fib(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fib(n-1) + fib(n-2),
    }
}

fn fib_benchmark(c: &mut Criterion) {
    let arr: Vec<u64> = (1..=30).cycle().take(480).collect();

    c.bench_function("fib sequential", |b| b.iter(|| {
        let _: Vec<_> = black_box(&arr).iter().map(|n| fib(*n)).collect();
    }));

    c.bench_function("fib parallel", |b| b.iter(|| {
        let _: Vec<_> = black_box(&arr).par_iter().map(|n| fib(*n)).collect();
    }));
}

criterion_group!(benches, fib_benchmark);
criterion_main!(benches);