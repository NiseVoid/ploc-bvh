//! A Bounding Volume Hierarchy based on PLOC.
//! Inspired by a series of articles on https://madmann91.github.io/

#![warn(missing_docs)]

// TODO: dim4?

mod morton;
mod search;

pub mod dim2;
pub mod dim3;

/// A generic BVH, can support any dimension that gets an implementation.
pub struct Bvh<D: Dim, T: Copy> {
    nodes: Vec<BvhNode<D>>,
    items: Vec<BvhItem<D, T>>,
}

impl<D: Dim, T: Copy> Default for Bvh<D, T> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            items: Vec::new(),
        }
    }
}

impl<D: Dim, T: Copy> Bvh<D, T> {
    /// Get the number of nodes in the BVH. The number of nodes is somewhere between
    /// the number of items (n) and n^2 - 1
    pub fn n_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Get the number of items in the BVH
    pub fn n_items(&self) -> usize {
        self.items.len()
    }
}

mod construct;
mod debug;

pub mod traverse;

pub mod intersect;
pub mod ray;
pub mod shape;

pub mod prelude {
    //! The prelude, exporting all the necessary things to get started

    pub use crate::{dim2::Bvh2d, dim3::Bvh3d, traverse::Stack};
}

use std::fmt::Debug;

/// A node on the BVH
#[derive(Clone, Copy, Debug)]
pub struct BvhNode<D: Dim> {
    aabb: D::Aabb,
    count: u32,
    start_index: u32,
}

/// An item in the BHV
#[derive(Clone, Copy, Debug)]
pub struct BvhItem<D: Dim, T: Copy> {
    aabb: D::Aabb,
    t: T,
}

/// A trait to generically handle different dimensionality
pub trait Dim: Copy + Debug {
    /// The AABB type used in this dimension
    type Aabb: Aabb;
}

/// A trait for a position type used in a dimension
pub trait Pos: Copy + Debug {
    /// The ZERO value for this type
    const ZERO: Self;
    /// Get the morton code for the position
    fn code(self) -> usize;
    /// Get the inverse of the position
    fn inverse(self) -> Self;
}

/// A trait for a generic AABB type
pub trait Aabb: Copy + Debug {
    /// The position type used by the AABB
    type Pos: Pos;

    /// An infinitely big AABB for this type
    const INFINITY: Self;
    /// The center of the AABB
    fn center(&self) -> Self::Pos;
    /// The area of the AABB
    fn area(&self) -> f32;
    /// Merge two AABBs, getting one that contains both
    fn merge(&self, other: &Self) -> Self;
    /// Get an AABB padded by the size of another AABB
    fn padded(&self, padding: &Self) -> Self;
    /// Get the intersection start and end times with a ray
    fn intersects_ray_at(&self, origin: Self::Pos, inv_dir: Self::Pos) -> (f32, f32);
    /// Check for intersections with another AABB
    fn intersects(&self, other: &Self) -> bool;
}
