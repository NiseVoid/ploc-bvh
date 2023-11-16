//! A module with generic logic for traversing the BVH

use crate::{Aabb, Bvh, BvhNode, Dim};

use std::collections::VecDeque;

/// A stack used when traversing the BVH, you can reuse this to save on an alloc
#[derive(Default)]
pub struct Stack(VecDeque<u32>);

impl std::ops::Deref for Stack {
    type Target = VecDeque<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<D: Dim, T: Copy> Bvh<D, T> {
    /// Create a stack with the right size for the BVH
    pub fn create_stack(&self) -> Stack {
        // TODO: Make sure we use the correct value here
        Stack(VecDeque::with_capacity(
            (self.items.len() as f32).log2().ceil() as usize + 10,
        ))
    }
}

/// A trait defining an intersection test to traverse the BVH
pub trait TraverseTest {
    /// The type of AABB supported by the test
    type Aabb: Aabb;

    /// The function to test an AABB
    fn test(&self, aabb: &Self::Aabb) -> bool;
}

/// A trait with generic getters for ray-based traversal methods
pub trait RayTest {
    /// the type of AABB supported by the test
    type Aabb: Aabb;

    /// Get the origin of the ray
    fn origin(&self) -> <Self::Aabb as Aabb>::Pos;
    /// Get the maximum length of the ray
    fn max(&self) -> f32;
}

/// An iterator that traverse the BVH using the provided [`TraverseTest`]
pub struct Traverser<'a, D: Dim, T: Copy, Test: TraverseTest> {
    bvh: &'a Bvh<D, T>,
    /// The test used in the traverser
    pub tester: Test,
    stack: &'a mut Stack,
    current_node: Option<u32>,
    offset: u32,
}

impl<'a, D: Dim, T: Copy, Test: TraverseTest<Aabb = D::Aabb>> Iterator
    for Traverser<'a, D, T, Test>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bvh.items.is_empty() {
            return None;
        }

        if let Some(current_node) = self.current_node {
            let node = &self.bvh.nodes[current_node as usize];
            match self.next_item(node) {
                None => {}
                v => return v,
            };
        }

        while let Some(index) = self.stack.pop_front() {
            let node = &self.bvh.nodes[index as usize];

            if !self.tester.test(&node.aabb) {
                continue;
            }

            if node.count > 0 {
                self.current_node = Some(index);
                self.offset = 0;

                match self.next_item(node) {
                    None => {}
                    v => return v,
                };
            } else {
                self.stack.push_back(node.start_index);
                self.stack.push_back(node.start_index + 1);
            }
        }

        None
    }
}

impl<'a, D: Dim, T: Copy, Test: TraverseTest<Aabb = D::Aabb>> Traverser<'a, D, T, Test> {
    #[inline(always)]
    fn next_item(&mut self, node: &'_ BvhNode<D>) -> Option<&'a T> {
        while self.current_node.is_some() {
            let item = &self.bvh.items[(node.start_index + self.offset) as usize];
            self.offset += 1;
            if self.offset == node.count {
                self.current_node = None;
            }
            if self.tester.test(&item.aabb) {
                return Some(&item.t);
            }
        }
        None
    }

    /// Construct a [`Traverser`]
    pub fn new(bvh: &'a Bvh<D, T>, stack: &'a mut Stack, tester: Test) -> Self {
        stack.clear();
        stack.reserve_exact((bvh.items.len() as f32).log2().ceil() as usize + 1);
        stack.push_back(0);
        Self {
            bvh,
            tester,
            stack,
            current_node: None,
            offset: 0,
        }
    }
}
