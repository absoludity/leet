use criterion::{black_box, criterion_group, criterion_main, Criterion};

use leet::trapping_rain_water_v3::Solution;

fn criterion_benchmark(c: &mut Criterion) {
    let heights_12 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

    let heights_1000 = include_str!("data/data_1000.txt")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let heights_10000: Vec<_> = include_str!("data/data_10000.txt")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    c.bench_function("Trapv3 with 12 inputs", |b| {
        b.iter(|| {
            Solution::trap(black_box(&heights_12));
        })
    });

    c.bench_function("Trapv3 with 1000 inputs", |b| {
        b.iter(|| {
            Solution::trap(black_box(&heights_1000));
        })
    });

    c.bench_function("Trapv3 with 10000 inputs", |b| {
        b.iter(|| {
            Solution::trap(black_box(&heights_10000));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
