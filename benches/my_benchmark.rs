
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hellors::{empty_instruction, fibonacci, fibonacci_cache};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 40", |b| b.iter(|| fibonacci(black_box(40))));
}

pub fn fibonacci_cached_benchmark(c: &mut Criterion) {
    c.bench_function("fib cached 40", |b| b.iter(|| fibonacci_cache(black_box(40))));
}

pub fn empty_instruction_benchmark(c: &mut Criterion) {
    c.bench_function("empty instruction", |b|{b.iter(|| empty_instruction())});
}

criterion_group!(benches, criterion_benchmark, fibonacci_cached_benchmark,empty_instruction_benchmark);
criterion_main!(benches);
