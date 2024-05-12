# Bench results

## Return FizzBuzzResult

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz, FizzBuzzAnswer};
use rayon::prelude::*;

static TEST_SIZE: isize = 100_000;

#[inline]
fn for_loop() {
    for i in 1..TEST_SIZE {
        let _ = i.fizzbuzz();
    }
}

#[inline]
fn for_loop_with_vec_overhead() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let mut out: Vec<FizzBuzzAnswer> = vec![];
    for i in inputs.into_iter() {
        out.push(i.fizzbuzz());
    }
}

#[inline]
fn vec_iter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<_> = inputs.iter().map(|num| num.fizzbuzz()).collect();
}

#[inline]
fn vec_intoiter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<_> = inputs.into_iter().map(|num| num.fizzbuzz()).collect();
}

#[inline]
fn vec_pariter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<_> = inputs.par_iter().map(|num| num.fizzbuzz()).collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("for_loop", |b| b.iter(|| for_loop()));
    c.bench_function("for_loop_with_vec_overhead", |b| b.iter(|| for_loop_with_vec_overhead()));
    c.bench_function("vec_iter", |b| b.iter(|| vec_iter()));
    c.bench_function("vec_intoiter", |b| b.iter(|| vec_intoiter()));
    c.bench_function("vec_pariter", |b| b.iter(|| vec_pariter()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 100_000

```text
for_loop                time:   [3.8837 ms 3.9661 ms 4.0560 ms]
                        change: [-89.983% -89.725% -89.423%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  10 (10.00%) high mild
  2 (2.00%) high severe

for_loop_with_vec_overhead
                        time:   [10.704 ms 10.968 ms 11.261 ms]
                        change: [-89.669% -89.360% -89.003%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  10 (10.00%) high mild
  8 (8.00%) high severe

vec_iter                time:   [8.1263 ms 8.4127 ms 8.7385 ms]
                        change: [-91.969% -91.592% -91.191%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

vec_intoiter            time:   [7.9589 ms 8.1105 ms 8.2829 ms]
                        change: [-91.986% -91.778% -91.569%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

vec_pariter             time:   [14.695 ms 15.173 ms 15.655 ms]
                        change: [-77.235% -76.143% -75.036%] (p = 0.00 < 0.05)
                        Performance has improved.

(.venv) fizzbuzz@c10ea50c1b09:/workspaces/FizzBuzz/fizzbuzz$ cargo bench
    Finished `bench` profile [optimized] target(s) in 0.07s
     Running unittests src/lib.rs (/workspaces/FizzBuzz/target/release/deps/fizzbuzz-13c63556c696a82b)

running 2 tests
test test::vec_not_consumed ... ignored
test test::vec_to_string ... ignored

test result: ok. 0 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench_fizzbuzz.rs (/workspaces/FizzBuzz/target/release/deps/bench_fizzbuzz-8abdaec00f9e9500)
Gnuplot not found, using plotters backend
for_loop                time:   [3.8195 ms 3.8918 ms 3.9720 ms]
                        change: [-4.5683% -1.8731% +1.1458%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe

for_loop_with_vec_overhead
                        time:   [10.528 ms 10.751 ms 11.004 ms]
                        change: [-5.2078% -1.9784% +1.6322%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  10 (10.00%) high mild
  5 (5.00%) high severe

vec_iter                time:   [7.4323 ms 7.6547 ms 7.9299 ms]
                        change: [-13.379% -9.0097% -4.6083%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

vec_intoiter            time:   [7.6105 ms 7.8016 ms 8.0196 ms]
                        change: [-6.7187% -3.8090% -0.5501%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe

vec_pariter             time:   [14.842 ms 15.345 ms 15.863 ms]
                        change: [-3.4012% +1.1285% +6.0992%] (p = 0.64 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
```

### 1_000_000

```text
for_loop                time:   [38.937 ms 39.793 ms 40.722 ms]
                        change: [+892.31% +922.48% +951.24%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

Benchmarking for_loop_with_vec_overhead: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.0s, or reduce sample count to 40.
for_loop_with_vec_overhead
                        time:   [95.979 ms 97.596 ms 99.355 ms]
                        change: [+782.21% +807.78% +832.90%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking vec_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.2s, or reduce sample count to 40.
vec_iter                time:   [97.266 ms 98.922 ms 100.69 ms]
                        change: [+1142.3% +1192.3% +1237.3%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking vec_intoiter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.0s, or reduce sample count to 40.
vec_intoiter            time:   [96.136 ms 97.858 ms 99.784 ms]
                        change: [+1112.7% +1154.3% +1192.6%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

vec_pariter             time:   [61.079 ms 63.132 ms 65.186 ms]
                        change: [+292.90% +311.43% +331.23%] (p = 0.00 < 0.05)
                        Performance has regressed.
```

### 10_000_000

```text
Benchmarking for_loop: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 40.8s, or reduce sample count to 10.
for_loop                time:   [386.66 ms 389.86 ms 393.18 ms]
                        change: [+856.27% +879.72% +902.72%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking for_loop_with_vec_overhead: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 99.1s, or reduce sample count to 10.
for_loop_with_vec_overhead
                        time:   [989.32 ms 1.0008 s 1.0127 s]
                        change: [+903.47% +925.50% +947.24%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking vec_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 100.3s, or reduce sample count to 10.
vec_iter                time:   [904.86 ms 913.62 ms 922.55 ms]
                        change: [+804.54% +823.58% +841.34%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking vec_intoiter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 85.2s, or reduce sample count to 10.
vec_intoiter            time:   [818.99 ms 826.11 ms 833.50 ms]
                        change: [+726.12% +744.19% +760.52%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Benchmarking vec_pariter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 57.5s, or reduce sample count to 10.
vec_pariter             time:   [382.34 ms 387.55 ms 392.61 ms]
                        change: [+492.74% +513.87% +536.02%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) low mild
  1 (1.00%) high mild
```

## Return String

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz};
use rayon::prelude::*;

static TEST_SIZE: isize = 100_000;

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
    let _: Vec<String> = inputs.into_iter().map(|num| num.fizzbuzz().into()).collect();
}

#[inline]
fn vec_pariter() {
    let inputs: Vec<_> = (1..TEST_SIZE).collect();
    let _: Vec<String> = inputs.par_iter().map(|num| num.fizzbuzz().into()).collect();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("for_loop", |b| b.iter(|| for_loop()));
    c.bench_function("for_loop_with_vec_overhead", |b| b.iter(|| for_loop_with_vec_overhead()));
    c.bench_function("vec_iter", |b| b.iter(|| vec_iter()));
    c.bench_function("vec_intoiter", |b| b.iter(|| vec_intoiter()));
    c.bench_function("vec_pariter", |b| b.iter(|| vec_pariter()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 100_000

```text
for_loop                time:   [4.3261 ms 4.4395 ms 4.5679 ms]
                        change: [+10.161% +14.014% +18.197%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe

for_loop_with_vec_overhead
                        time:   [11.089 ms 11.465 ms 11.916 ms]
                        change: [+5.7443% +11.000% +17.067%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

vec_iter                time:   [7.8512 ms 8.0857 ms 8.3479 ms]
                        change: [-3.4048% +0.9937% +5.5594%] (p = 0.66 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

vec_intoiter            time:   [7.5814 ms 7.7746 ms 8.0153 ms]
                        change: [-5.4652% -2.3848% +0.8759%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe

vec_pariter             time:   [14.346 ms 14.729 ms 15.116 ms]
                        change: [-14.415% -10.728% -6.8381%] (p = 0.00 < 0.05)
                        Performance has improved.
```

### 1_000_000

```text
for_loop                time:   [43.455 ms 44.500 ms 45.739 ms]
                        change: [+867.32% +902.39% +941.10%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

Benchmarking for_loop_with_vec_overhead: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.9s, or reduce sample count to 50.
for_loop_with_vec_overhead
                        time:   [96.174 ms 98.352 ms 100.78 ms]
                        change: [+720.66% +757.83% +794.82%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Benchmarking vec_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.0s, or reduce sample count to 50.
vec_iter                time:   [91.775 ms 93.762 ms 96.066 ms]
                        change: [+1016.5% +1059.6% +1105.9%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

Benchmarking vec_intoiter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, or reduce sample count to 50.
vec_intoiter            time:   [94.280 ms 96.871 ms 99.813 ms]
                        change: [+1096.1% +1146.0% +1196.5%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe

vec_pariter             time:   [55.747 ms 57.200 ms 58.714 ms]
                        change: [+274.43% +288.36% +303.41%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

## Finding break-even

### 300_000

```text
vec_iter                time:   [23.252 ms 23.915 ms 24.645 ms]
                        change: [-75.400% -74.494% -73.522%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

vec_pariter             time:   [24.358 ms 25.161 ms 25.992 ms]
                        change: [-57.684% -56.013% -54.132%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```


### 250_000

```text
vec_iter                time:   [18.911 ms 19.349 ms 19.850 ms]
                        change: [-22.174% -19.095% -16.151%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe

vec_pariter             time:   [26.470 ms 27.406 ms 28.384 ms]
                        change: [+3.9135% +8.9233% +14.506%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
```

### 275_000

```text
vec_iter                time:   [20.926 ms 21.342 ms 21.792 ms]
                        change: [+6.8293% +10.300% +13.824%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  10 (10.00%) high mild
  1 (1.00%) high severe

vec_pariter             time:   [23.185 ms 23.981 ms 24.823 ms]
                        change: [-16.495% -12.497% -7.8280%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```

## Using MultiFizzBuzz trait, with par_iter when >300_000 elements

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use fizzbuzz::{self, FizzBuzz, MultiFizzBuzz};
use rayon::prelude::*;

static TEST_SIZE: isize = 100_000;

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
    let _: Vec<String> = inputs.into_iter().map(|num| num.fizzbuzz().into()).collect();
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
    c.bench_function("for_loop_with_vec_overhead", |b| b.iter(|| for_loop_with_vec_overhead()));
    c.bench_function("vec_iter", |b| b.iter(|| vec_iter()));
    c.bench_function("vec_intoiter", |b| b.iter(|| vec_intoiter()));
    c.bench_function("vec_pariter", |b| b.iter(|| vec_pariter()));
    c.bench_function("multifizzbuzz_trait", |b| b.iter(|| multifizzbuzz_trait()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 100_000

```text
for_loop                time:   [4.2707 ms 4.3504 ms 4.4401 ms]
                        change: [-90.538% -90.224% -89.916%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe

for_loop_with_vec_overhead
                        time:   [11.099 ms 11.428 ms 11.789 ms]
                        change: [-88.836% -88.381% -87.900%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

vec_iter                time:   [7.7721 ms 8.0990 ms 8.4777 ms]
                        change: [-63.641% -62.051% -60.088%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe

vec_intoiter            time:   [7.4831 ms 7.6820 ms 7.9091 ms]
                        change: [-92.383% -92.070% -91.727%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe

vec_pariter             time:   [16.626 ms 17.076 ms 17.532 ms]
                        change: [-31.716% -28.794% -25.604%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

multifizzbuzz_trait     time:   [7.8593 ms 8.0643 ms 8.2940 ms]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
```

### 1_000_000

```text
for_loop                time:   [43.583 ms 44.555 ms 45.632 ms]
                        change: [+894.93% +924.17% +957.38%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 17 outliers among 100 measurements (17.00%)
  10 (10.00%) high mild
  7 (7.00%) high severe

Benchmarking for_loop_with_vec_overhead: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.9s, or reduce sample count to 40.
for_loop_with_vec_overhead
                        time:   [97.879 ms 99.476 ms 101.23 ms]
                        change: [+739.76% +770.48% +800.07%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking vec_iter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.9s, or reduce sample count to 50.
vec_iter                time:   [91.757 ms 94.214 ms 97.209 ms]
                        change: [+1000.5% +1063.3% +1127.1%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

Benchmarking vec_intoiter: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 10.0s, or reduce sample count to 40.
vec_intoiter            time:   [93.728 ms 95.574 ms 97.608 ms]
                        change: [+1100.8% +1144.1% +1187.1%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

vec_pariter             time:   [57.161 ms 58.950 ms 60.776 ms]
                        change: [+230.76% +245.22% +258.85%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

multifizzbuzz_trait     time:   [56.362 ms 57.859 ms 59.375 ms]
                        change: [+590.60% +617.47% +644.82%] (p = 0.00 < 0.05)
                        Performance has regressed.
```
