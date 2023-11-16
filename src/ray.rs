//! A module implementing a raycasting traversal method for the BVH

use crate::traverse::{RayTest, Stack, TraverseTest, Traverser};
use crate::{
    dim2::{Aabb2d, Dim2},
    dim3::{Aabb3d, Dim3},
};
use crate::{Aabb, Bvh, Pos};

impl<'a, T: Copy> Bvh<Dim2, T> {
    /// Traverse the BVH using a ray cast
    pub fn cast_ray(
        &'a self,
        stack: &'a mut Stack,
        origin: <Aabb2d as Aabb>::Pos,
        direction: <Aabb2d as Aabb>::Pos,
        max_toi: f32,
    ) -> Traverser<'a, Dim2, T, RayTester<Aabb2d>> {
        Traverser::new(self, stack, RayTester::new(origin, direction, max_toi))
    }
}

impl<'a, T: Copy> Bvh<Dim3, T> {
    /// Traverse the BVH using a ray cast
    pub fn cast_ray(
        &'a self,
        stack: &'a mut Stack,
        origin: <Aabb3d as Aabb>::Pos,
        direction: <Aabb3d as Aabb>::Pos,
        max_toi: f32,
    ) -> Traverser<'a, Dim3, T, RayTester<Aabb3d>> {
        Traverser::new(self, stack, RayTester::new(origin, direction, max_toi))
    }
}

pub(crate) fn intersects_ray<A: Aabb>(aabb: &A, origin: A::Pos, inv_dir: A::Pos, max: f32) -> bool {
    let (tmin, tmax) = aabb.intersects_ray_at(origin, inv_dir);

    let tmin = tmin.max(0.);
    let tmax = tmax.min(max);

    tmin <= tmax
}

/// A ray-based traversal method for the BVH
pub struct RayTester<A: Aabb> {
    /// The origin of the ray
    pub origin: A::Pos,
    inv_dir: A::Pos,
    /// The maximum length of the ray
    pub max: f32,
}

impl TraverseTest for RayTester<Aabb2d> {
    type Aabb = Aabb2d;

    fn test(&self, aabb: &Self::Aabb) -> bool {
        intersects_ray(aabb, self.origin, self.inv_dir, self.max)
    }
}

impl TraverseTest for RayTester<Aabb3d> {
    type Aabb = Aabb3d;

    fn test(&self, aabb: &Self::Aabb) -> bool {
        intersects_ray(aabb, self.origin, self.inv_dir, self.max)
    }
}

impl<A: Aabb> RayTest for RayTester<A> {
    type Aabb = A;

    fn origin(&self) -> <Self::Aabb as Aabb>::Pos {
        self.origin
    }

    fn max(&self) -> f32 {
        self.max
    }
}

impl<A: Aabb> RayTester<A> {
    fn new(origin: A::Pos, direction: A::Pos, max: f32) -> Self {
        Self {
            origin,
            inv_dir: direction.inverse(),
            max,
        }
    }
}
