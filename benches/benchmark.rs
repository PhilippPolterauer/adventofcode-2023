use adventofcode::days::*;
use adventofcode::util::load_file;
use criterion::{criterion_group, criterion_main, Criterion};

#[macro_export]
macro_rules! make_benchmark {
    ($c:expr, $dayn:expr, $part:expr) => {
        let input = load_file($dayn, $part, false, "data");
        let func = dispatch_function($dayn, $part);
        $c.bench_function(&format!("day{}::part{}", $dayn, $part), |b| {
            b.iter(|| func(&input))
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    make_benchmark!(c, 1, 1);
    make_benchmark!(c, 1, 2);
    make_benchmark!(c, 2, 1);
    make_benchmark!(c, 2, 2);
    make_benchmark!(c, 3, 1);
    make_benchmark!(c, 3, 2);
    make_benchmark!(c, 4, 1);
    make_benchmark!(c, 4, 2);
    make_benchmark!(c, 5, 1);
    make_benchmark!(c, 5, 2);
    make_benchmark!(c, 6, 1);
    make_benchmark!(c, 6, 2);
    make_benchmark!(c, 7, 1);
    make_benchmark!(c, 7, 2);
    make_benchmark!(c, 8, 1);
    make_benchmark!(c, 8, 2);
    make_benchmark!(c, 9, 1);
    make_benchmark!(c, 9, 2);
    make_benchmark!(c, 10, 1);
    make_benchmark!(c, 10, 2);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
