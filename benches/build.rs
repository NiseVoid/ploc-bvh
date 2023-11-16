use ploc_bvh::prelude::Bvh3d;

use std::time::{Duration, Instant};

use bevy_math::Vec3;
use criterion::{criterion_group, criterion_main, Criterion};

const N_BOXES: usize = 1000;

fn generate_boxes() -> Vec<(Vec3, Vec3)> {
    fastrand::seed(1);

    let mut boxes = Vec::with_capacity(N_BOXES);
    for _ in 0..boxes.capacity() {
        let pos = Vec3::new(
            fastrand::f32() * 50. - 25.,
            fastrand::f32() * 50. - 25.,
            fastrand::f32() * 50. - 25.,
        );

        let aabb = Vec3::new(
            fastrand::f32() * 8. + 2.,
            fastrand::f32() * 8. + 2.,
            fastrand::f32() * 8. + 2.,
        );

        boxes.push((aabb / -2. + pos, aabb / 2. + pos));
    }

    boxes
}

fn build(c: &mut Criterion) {
    c.bench_function("many boxes", |b| {
        b.iter_custom(|iter| {
            let boxes = generate_boxes();
            let mut elapsed = Duration::ZERO;
            for _ in 0..iter {
                let start = Instant::now();
                let bvh = Bvh3d::new(
                    boxes.len(),
                    boxes.iter().enumerate().map(|(i, aabb)| (i as u32, *aabb)),
                );
                elapsed += start.elapsed();
                assert_eq!(bvh.n_items(), N_BOXES);
                assert!(bvh.n_nodes() < N_BOXES * 2);
            }

            elapsed
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = build
}
criterion_main!(benches);
