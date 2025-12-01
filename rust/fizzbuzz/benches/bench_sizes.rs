#![allow(dead_code)]
use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzzAnswer, MultiFizzBuzz};
use rayon::iter::ParallelIterator;

#[inline]
fn range_20() {
    const TEST_SIZE: i32 = 20;
    let _: Vec<FizzBuzzAnswer> = (1..TEST_SIZE).fizzbuzz().collect();
}

#[inline]
fn range_200_000() {
    const TEST_SIZE: i32 = 200_000;
    let _: Vec<FizzBuzzAnswer> = (1..TEST_SIZE).fizzbuzz().collect();
}
#[inline]
fn range_300_000() {
    const TEST_SIZE: i32 = 300_000;
    let _: Vec<FizzBuzzAnswer> = (1..TEST_SIZE).fizzbuzz().collect();
}
#[inline]
fn range_1_000_000() {
    const TEST_SIZE: i32 = 1_000_000;
    let _: Vec<FizzBuzzAnswer> = (1..TEST_SIZE).fizzbuzz().collect();
}
#[inline]
fn range_10_000_000() {
    const TEST_SIZE: i32 = 10_000_000;
    let _: Vec<FizzBuzzAnswer> = (1..TEST_SIZE).fizzbuzz().collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("20", |b| b.iter(range_20));
    c.bench_function("200_000", |b| b.iter(range_200_000));
    c.bench_function("300_000", |b| b.iter(range_300_000));
    c.bench_function("1_000_000", |b| b.iter(range_1_000_000));
    c.bench_function("10_000_000", |b| b.iter(range_10_000_000));
}

criterion_group!(sizes, criterion_benchmark);
criterion_main!(sizes);
