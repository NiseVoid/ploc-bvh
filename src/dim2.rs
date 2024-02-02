//! A module with implementations for 2D support

use crate::morton::{morton_encode, MORTON_CENTER};
use crate::{Bvh, BvhVolume};

pub use bevy_math::{
    bounding::{Aabb2d, BoundingCircle, BoundingVolume},
    Vec2,
};

/// A BVH using [`Aabb2d`] volumes
pub type BvhAabb2d<T> = Bvh<Aabb2d, T>;

/// A BVH using [`BoundingCircle`] volumes
pub type BvhCircle<T> = Bvh<BoundingCircle, T>;

impl BvhVolume for Aabb2d {
    const INFINITY: Self = Self {
        min: Vec2::splat(-f32::INFINITY),
        max: Vec2::splat(f32::INFINITY),
    };

    #[inline(always)]
    fn morton_code(&self) -> usize {
        let center = self.center();
        morton_encode(
            (center.x + MORTON_CENTER) as usize,
            (center.y + MORTON_CENTER) as usize,
            0,
            5,
        )
    }
}

impl BvhVolume for BoundingCircle {
    const INFINITY: Self = Self {
        center: Vec2::ZERO,
        circle: bevy_math::primitives::Circle {
            radius: f32::INFINITY,
        },
    };

    #[inline(always)]
    fn morton_code(&self) -> usize {
        let center = self.center();
        morton_encode(
            (center.x + MORTON_CENTER) as usize,
            (center.y + MORTON_CENTER) as usize,
            0,
            5,
        )
    }
}
