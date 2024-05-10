use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz, MultiFizzBuzz};

#[inline]
fn fb100_000() {
    for _ in 1..10 {
        for i in 1..100_000 {
            i.fizzbuzz();
        }
    }
}

#[inline]
fn fb100_000_vec() {
    for _ in 1..10 {
        (1..100_000).collect::<Vec<_>>().fizzbuzz();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fizzbuzz 100_000", |b| b.iter(|| fb100_000()));
    c.bench_function("fizzbuzz 100_000_vector", |b| b.iter(|| fb100_000_vec()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
