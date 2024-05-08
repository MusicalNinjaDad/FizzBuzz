use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz};

#[inline]
fn fb100_000() {
    for _ in 1..10 {
        for i in 1..100_000 {
            i.fizzbuzz();
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fizzbuzz 100_000", |b| b.iter(|| fb100_000()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
