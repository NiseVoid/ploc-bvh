use ploc_bvh::prelude::BvhAabb3d;

use std::time::{Duration, Instant};

use bevy_math::{bounding::Aabb3d, Vec3};
use criterion::{criterion_group, criterion_main, Criterion};

const N_BOXES: usize = 1000;

fn generate_boxes() -> Vec<Aabb3d> {
    fastrand::seed(1);

    let mut boxes = Vec::with_capacity(N_BOXES);
    for _ in 0..boxes.capacity() {
        let pos = Vec3::new(
            fastrand::f32() * 50. - 25.,
            fastrand::f32() * 50. - 25.,
            fastrand::f32() * 50. - 25.,
        );

        let half_size = Vec3::new(
            fastrand::f32() * 4. + 1.,
            fastrand::f32() * 4. + 1.,
            fastrand::f32() * 4. + 1.,
        );

        boxes.push(Aabb3d::new(pos, half_size));
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
                let bvh = BvhAabb3d::new(
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
