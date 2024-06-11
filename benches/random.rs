use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use fastrand;

fn bench_rand(c: &mut Criterion) {
    c.bench_function("rand::thread_rng", |b| {
        b.iter(|| {
            let mut rng = rand::thread_rng();
            let _num: f64 = rng.gen();
        })
    });
}

fn bench_fastrand(c: &mut Criterion) {
    c.bench_function("fastrand", |b| {
        b.iter(|| {
            let _num: f64 = fastrand::f64();
        })
    });
}

criterion_group!(benches, bench_rand, bench_fastrand);
criterion_main!(benches);
