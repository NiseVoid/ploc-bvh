//! A module defining an AABB intersection traversal test

use crate::traverse::{Stack, TraverseTest, Traverser};
use crate::{Aabb, Bvh, Dim};

impl<'a, D: Dim, T: Copy> Bvh<D, T> {
    /// Get an iterator traversing the BVH based on an intersection test
    pub fn intersect_aabb(
        &'a self,
        stack: &'a mut Stack,
        aabb: D::Aabb,
    ) -> Traverser<'a, D, T, IntersectTester<D::Aabb>> {
        Traverser::new(self, stack, IntersectTester::new(aabb))
    }
}

/// An AABB intersection test to traverse the BVH
pub struct IntersectTester<A> {
    aabb: A,
}

impl<A: Aabb> TraverseTest for IntersectTester<A> {
    type Aabb = A;

    #[inline(always)]
    fn test(&self, aabb: &A) -> bool {
        self.aabb.intersects(aabb)
    }
}

impl<A: Aabb> IntersectTester<A> {
    fn new(aabb: A) -> Self {
        Self { aabb }
    }
}
