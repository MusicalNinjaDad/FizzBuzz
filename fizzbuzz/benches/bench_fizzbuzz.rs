use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz, MultiFizzBuzz};
use rayon::prelude::*;

static TEST_SIZE: isize = 1_000_000;

#[inline]
fn for_loop() {
    for i in 1..TEST_SIZE {
        let _: String = i.fizzbuzz().into();
    }
}

#[inline]
fn for_loop_with_vec_overhead() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let mut out: Vec<String> = vec![];
    for i in inputs.into_iter() {
        out.push(i.fizzbuzz().into());
    }
}

#[inline]
fn vec_iter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<String> = inputs.iter().map(|num| num.fizzbuzz().into()).collect();
}

#[inline]
fn vec_intoiter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<String> = inputs
        .into_iter()
        .map(|num| num.fizzbuzz().into())
        .collect();
}

#[inline]
fn vec_pariter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<String> = inputs.par_iter().map(|num| num.fizzbuzz().into()).collect();
}

#[inline]
fn multifizzbuzz_trait() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<String> = inputs.fizzbuzz().into();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("for_loop", |b| b.iter(|| for_loop()));
    c.bench_function("for_loop_with_vec_overhead", |b| {
        b.iter(|| for_loop_with_vec_overhead())
    });
    c.bench_function("vec_iter", |b| b.iter(|| vec_iter()));
    c.bench_function("vec_intoiter", |b| b.iter(|| vec_intoiter()));
    c.bench_function("vec_pariter", |b| b.iter(|| vec_pariter()));
    c.bench_function("multifizzbuzz_trait", |b| b.iter(|| multifizzbuzz_trait()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
