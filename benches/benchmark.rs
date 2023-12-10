use adventofcode::days::day1;
use adventofcode::util::load_file;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = load_file(2, 1, false, "data");

    c.bench_function("day1::part1", |b| {
        b.iter(|| {
            day1::part1(&input);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
