//! A crate implementing 3D support for the BVH

use crate::morton::{morton_encode, MORTON_CENTER};
use crate::{Bvh, BvhVolume};

pub use bevy_math::{
    bounding::{Aabb3d, BoundingSphere, BoundingVolume},
    Vec3,
};

/// A BVH using [`Aabb3d`] volumes
pub type BvhAabb3d<T> = Bvh<Aabb3d, T>;

/// A BVH using [`BoundingSphere`] volumes
pub type BvhSphere<T> = Bvh<BoundingSphere, T>;

impl BvhVolume for Aabb3d {
    const INFINITY: Self = Self {
        min: Vec3::splat(-f32::INFINITY),
        max: Vec3::splat(f32::INFINITY),
    };

    #[inline(always)]
    fn morton_code(&self) -> usize {
        let center = self.center();
        morton_encode(
            (center.x + MORTON_CENTER) as usize,
            (center.y + MORTON_CENTER) as usize,
            (center.z + MORTON_CENTER) as usize,
            5,
        )
    }
}

impl BvhVolume for BoundingSphere {
    const INFINITY: Self = Self {
        center: Vec3::ZERO,
        sphere: bevy_math::primitives::Sphere {
            radius: f32::INFINITY,
        },
    };

    #[inline(always)]
    fn morton_code(&self) -> usize {
        let center = self.center();
        morton_encode(
            (center.x + MORTON_CENTER) as usize,
            (center.y + MORTON_CENTER) as usize,
            (center.z + MORTON_CENTER) as usize,
            5,
        )
    }
}
