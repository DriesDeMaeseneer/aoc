use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use lib::euler1; // function to profile
use aoc_2022::day_1::max_calories;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day_1", |b| {
        b.iter(|| max_calories(black_box("day_1.input"), black_box(3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
