//! A module that implements a shape-casting traversal method.
//! It's really just a raycasting traverser but it pads all AABBs by the shape it's using

use crate::ray::intersects_ray;
use crate::traverse::{RayTest, Stack, TraverseTest, Traverser};
use crate::{
    dim2::{Aabb2d, Dim2},
    dim3::{Aabb3d, Dim3},
};
use crate::{Aabb, Bvh, Pos};

impl<'a, T: Copy> Bvh<Dim2, T> {
    /// Traverse the BVH with a shape cast
    pub fn cast_shape(
        &'a self,
        stack: &'a mut Stack,
        shape_aabb: Aabb2d,
        origin: <Aabb2d as Aabb>::Pos,
        direction: <Aabb2d as Aabb>::Pos,
        max_toi: f32,
    ) -> Traverser<'a, Dim2, T, ShapeTester<Aabb2d>> {
        Traverser::new(
            self,
            stack,
            ShapeTester::new(shape_aabb, origin, direction, max_toi),
        )
    }
}

impl<'a, T: Copy> Bvh<Dim3, T> {
    /// Traverse the BVH with a shape cast
    pub fn cast_shape(
        &'a self,
        stack: &'a mut Stack,
        shape_aabb: Aabb3d,
        origin: <Aabb3d as Aabb>::Pos,
        direction: <Aabb3d as Aabb>::Pos,
        max_toi: f32,
    ) -> Traverser<'a, Dim3, T, ShapeTester<Aabb3d>> {
        Traverser::new(
            self,
            stack,
            ShapeTester::new(shape_aabb, origin, direction, max_toi),
        )
    }
}

/// A shape-casting based traversal method
pub struct ShapeTester<A: Aabb> {
    shape_aabb: A,
    /// The origin of the shape cast
    pub origin: A::Pos,
    inv_dir: A::Pos,
    /// The maximum distance of the shape cast
    pub max: f32,
}

impl TraverseTest for ShapeTester<Aabb2d> {
    type Aabb = Aabb2d;

    fn test(&self, aabb: &Self::Aabb) -> bool {
        intersects_ray(
            &aabb.padded(&self.shape_aabb),
            self.origin,
            self.inv_dir,
            self.max,
        )
    }
}

impl TraverseTest for ShapeTester<Aabb3d> {
    type Aabb = Aabb3d;

    fn test(&self, aabb: &Self::Aabb) -> bool {
        intersects_ray(
            &aabb.padded(&self.shape_aabb),
            self.origin,
            self.inv_dir,
            self.max,
        )
    }
}

impl<A: Aabb> RayTest for ShapeTester<A> {
    type Aabb = A;

    fn origin(&self) -> <Self::Aabb as Aabb>::Pos {
        self.origin
    }

    fn max(&self) -> f32 {
        self.max
    }
}

impl<A: Aabb> ShapeTester<A> {
    fn new(shape_aabb: A, origin: A::Pos, direction: A::Pos, max: f32) -> Self {
        Self {
            shape_aabb,
            origin,
            inv_dir: direction.inverse(),
            max,
        }
    }
}
