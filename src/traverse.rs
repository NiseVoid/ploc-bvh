//! A module with generic logic for traversing the BVH

use crate::{Bvh, BvhNode, BvhVolume};

use std::collections::VecDeque;

use bevy_math::bounding::IntersectsVolume;

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

impl<Volume: BvhVolume, T: Copy> Bvh<Volume, T> {
    /// Create a stack with the right size for the BVH
    pub fn create_stack(&self) -> Stack {
        // TODO: Make sure we use the correct value here
        Stack(VecDeque::with_capacity(
            (self.items.len() as f32).log2().ceil() as usize + 10,
        ))
    }

    /// Traverse the BVH with the provided [`IntersectsVolume`] test
    pub fn traverse<'a, Test: IntersectsVolume<Volume>>(
        &'a self,
        stack: &'a mut Stack,
        tester: Test,
    ) -> Traverser<'a, Volume, T, Test> {
        stack.clear();
        stack.reserve_exact((self.items.len() as f32).log2().ceil() as usize + 10);
        stack.push_back(0);

        Traverser {
            bvh: self,
            tester,
            stack,
            current_node: None,
            offset: 0,
        }
    }
}

/// An iterator that traverse the BVH using the provided [`IntersectsVolume`] test
pub struct Traverser<'a, Volume: BvhVolume, T: Copy, Test: IntersectsVolume<Volume>> {
    bvh: &'a Bvh<Volume, T>,
    /// The test used in the traverser
    pub tester: Test,
    stack: &'a mut Stack,
    current_node: Option<u32>,
    offset: u32,
}

impl<'a, Volume: BvhVolume, T: Copy, Test: IntersectsVolume<Volume>> Iterator
    for Traverser<'a, Volume, T, Test>
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

            if !self.tester.intersects(&node.volume) {
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

impl<'a, Volume: BvhVolume, T: Copy, Test: IntersectsVolume<Volume>>
    Traverser<'a, Volume, T, Test>
{
    #[inline(always)]
    fn next_item(&mut self, node: &'_ BvhNode<Volume>) -> Option<&'a T> {
        while self.current_node.is_some() {
            let item = &self.bvh.items[(node.start_index + self.offset) as usize];
            self.offset += 1;
            if self.offset == node.count {
                self.current_node = None;
            }
            if self.tester.intersects(&item.volume) {
                return Some(&item.t);
            }
        }
        None
    }
}
