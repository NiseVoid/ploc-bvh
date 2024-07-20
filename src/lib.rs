#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod morton;
mod search;

pub mod dim2;
pub mod dim3;

/// A generic bounding volume supported by the BVH. Adds a few extra methods on top of bevy's
/// [`BoundingVolume`](bevy_math::bounding::BoundingVolume) trait
pub trait BvhVolume: bevy_math::bounding::BoundingVolume + Clone + Debug {
    /// An infinite bounding volume at the zero position
    const INFINITY: Self;

    /// Get the morton code for the center of the volume
    fn morton_code(&self) -> usize;
}

/// A generic BVH, can support any dimension that gets an implementation.
pub struct Bvh<Volume: BvhVolume, T: Copy> {
    nodes: Vec<BvhNode<Volume>>,
    items: Vec<BvhItem<Volume, T>>,
}

impl<Volume: BvhVolume, T: Copy> Default for Bvh<Volume, T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl<Volume: BvhVolume, T: Copy> Bvh<Volume, T> {
    /// Get the number of nodes in the BVH. The number of nodes is somewhere between
    /// the number of items (n) and 2n - 1
    pub fn n_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of items in the BVH
    pub fn n_items(&self) -> usize {
        self.items.len()
    }

    /// Get an iterator over the BVH's nodes
    pub fn nodes(&self) -> impl Iterator<Item = &BvhNode<Volume>> {
        self.nodes.iter()
    }

    /// Get an iterator over the BVH's items
    pub fn items(&self) -> impl Iterator<Item = &BvhItem<Volume, T>> {
        self.items.iter()
    }
}

mod construct;
mod debug;

pub mod traverse;

pub mod prelude {
    //! The prelude, exporting all the necessary things to get started

    pub use crate::{dim2::*, dim3::*, traverse::Stack};
}

use std::fmt::Debug;

/// A node on the BVH
#[derive(Clone, Copy, Debug)]
pub struct BvhNode<Volume: BvhVolume> {
    /// The volume of the node
    pub volume: Volume,
    /// The number of leaves. 0 if the node points to other nodes
    pub count: u32,
    /// The start index of the leaves. If count is 0 this points to other nodes
    pub start_index: u32,
}

/// An item in the BHV
#[derive(Clone, Copy, Debug)]
pub struct BvhItem<Volume: BvhVolume, T: Copy> {
    /// The volume of the item
    pub volume: Volume,
    /// The value of the bvh item
    pub t: T,
}
