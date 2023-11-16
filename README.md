# ploc-bhv

A Bounding Volume Hierarchy based on PLOC.
Inspired by [a series of articles by Arsène Pérard-Gayot](https://madmann91.github.io/)

## Getting started

Creating and traversing the BVH is all done using Iterators.

In this example we create AABBs for a few boxes, and use their index as the key:
```rust
use ploc_bvh::prelude::*;
use bevy_math::Vec3;

...

let boxes: Vec<(Vec3, Vec3)> = generate_boxes();
let bvh = Bvh3d::new(
    boxes.len(),
    boxes.iter().enumerate().map(|(i, aabb)| (i as u32, *aabb)),
);
```

Next we can simply iterate over the BVH using one of the provided methods:
```rust
let mut stack = bvh.create_stack();

let origin = Vec3::ZERO;
let direction = Vec3::Y;
let max_time_of_impact = 1.;
for index in bvh.cast_ray(&mut stack, origin, direction, max_time_of_impact) {
    println!("We hit box {}: {:?}", index, boxes[index]);
}
```

It's recommended to reuse the stack where possible to avoid unnecessary allocations.

## Future work

- Actually support the parallelization

## Licensing

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
