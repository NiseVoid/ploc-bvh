# ploc-bhv

A Bounding Volume Hierarchy based on PLOC.
Inspired by [a series of articles by Arsène Pérard-Gayot](https://madmann91.github.io/)

## Getting started

Creating and traversing the BVH is all done using Iterators.

In this example we create AABBs for a few boxes, and use their index as the key, then travese the BVH:
```rust
# use ploc_bvh::prelude::*;
use bevy_math::{bounding::{Aabb3d, RayCast3d}, prelude::{Direction3d, Vec3}};

// We have some list of axis-aligned bounding boxes
let boxes = [
    Aabb3d::new(Vec3::ONE, Vec3::ONE),
    Aabb3d::new(Vec3::NEG_Y, Vec3::splat(2.)),
];

// We build a 3D BVH using the number of boxes, and an iterator of (T, Aabb3d).
// T can be whatever type we need, but it most likely includes some identifier,
// and maybe some information to filter results quickly.
let bvh = BvhAabb3d::new(
    boxes.len(),
    boxes.iter().enumerate().map(|(i, aabb)| (i, *aabb)),
);

// Next we want to traverse the BVH, to do this we need a stack and an intersection test.

// We can create a stack, this type can be reused to save some allocs if necessary.
let mut stack = bvh.create_stack();

// We construct a bounding volume intersection test, a raycast in this case
let origin = Vec3::ZERO;
let direction = Direction3d::Y;
let max_time_of_impact = 1.;
let ray_cast = RayCast3d::new(origin, direction, max_time_of_impact);

// Now we can iterate over the BVH using the `traverse` method
for &index in bvh.traverse(&mut stack, ray_cast) {
    // The value returned from `traverse` matches the T used when constructing the BVH
    println!("We hit box {}: {:?}", index, boxes[index]);
}
```

## Future work

- Actually support the parallelization

## Licensing

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
