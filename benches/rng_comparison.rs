use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
};
use rand_xoshiro::Xoshiro256PlusPlus;

fn bench_rng_methods(c: &mut Criterion) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(395_992_934_456_271);
    let prev_height_bound = 10.;
    let height_bound = 50.;

    let mut group = c.benchmark_group("RNG Scaling");

    group.bench_function("random", |b| {
        b.iter(|| {
            let val = rng
                .random::<f32>()
                .mul_add(height_bound - prev_height_bound, prev_height_bound)
                * if rng.random_bool(0.5) { 1. } else { -1. };
            black_box(val);
        })
    });

    group.bench_function("range", |b| {
        let dist = Uniform::new(prev_height_bound, height_bound).unwrap();
        b.iter(|| {
            let val = dist.sample(&mut rng) * if rng.random_bool(0.5) { 1. } else { -1. };
            black_box(val);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_rng_methods);
criterion_main!(benches);
