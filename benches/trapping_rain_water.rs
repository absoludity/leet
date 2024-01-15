use criterion::{black_box, criterion_group, criterion_main, Criterion};

use leet::trapping_rain_water::Solution;

fn criterion_benchmark(c: &mut Criterion) {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    c.bench_function("Trap case 1", |b| {
        b.iter(|| {
            Solution::trap(black_box(&heights));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
