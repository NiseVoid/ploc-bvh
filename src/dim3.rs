//! A crate implementing 3D support for the BVH

use crate::morton::{morton_encode, MORTON_CENTER};
use crate::{Aabb, Bvh, Dim, Pos};

pub use bevy_math::Vec3;

/// A 3D BVH
pub type Bvh3d<T> = Bvh<Dim3, T>;

/// An dimension implementation for 3D
#[derive(Clone, Copy, Debug)]
pub struct Dim3;

impl Dim for Dim3 {
    type Aabb = Aabb3d;
}

impl Pos for Vec3 {
    const ZERO: Self = Self::ZERO;

    #[inline(always)]
    fn code(self) -> usize {
        morton_encode(
            (self.x + MORTON_CENTER) as usize,
            (self.y + MORTON_CENTER) as usize,
            (self.z + MORTON_CENTER) as usize,
            5,
        )
    }

    #[inline(always)]
    fn inverse(self) -> Self {
        Vec3::ONE / self
    }
}

/// A 3D Axis-Aligned Bounding Box
#[derive(Clone, Copy, Debug)]
pub struct Aabb3d {
    min: Vec3,
    max: Vec3,
}

impl From<(Vec3, Vec3)> for Aabb3d {
    #[inline(always)]
    fn from((min, max): (Vec3, Vec3)) -> Self {
        Self { min, max }
    }
}

impl Aabb for Aabb3d {
    type Pos = Vec3;

    const INFINITY: Self = Self {
        min: Vec3::splat(-f32::INFINITY),
        max: Vec3::splat(f32::INFINITY),
    };

    #[inline(always)]
    fn center(&self) -> Self::Pos {
        self.max - self.min
    }

    #[inline(always)]
    fn area(&self) -> f32 {
        let b = self.max - self.min;
        b.x * (b.y + b.z) + b.y * b.z
    }

    #[inline(always)]
    fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    #[inline(always)]
    fn padded(&self, padding: &Self) -> Self {
        Self {
            min: self.min + padding.min,
            max: self.max + padding.max,
        }
    }

    fn intersects_ray_at(&self, origin: Self::Pos, inv_dir: Self::Pos) -> (f32, f32) {
        // TODO: This might not be optimal
        let (min_x, max_x) = if inv_dir.x.is_sign_positive() {
            (self.min.x, self.max.x)
        } else {
            (self.max.x, self.min.x)
        };
        let (min_y, max_y) = if inv_dir.y.is_sign_positive() {
            (self.min.y, self.max.y)
        } else {
            (self.max.y, self.min.y)
        };
        let (min_z, max_z) = if inv_dir.z.is_sign_positive() {
            (self.min.z, self.max.z)
        } else {
            (self.max.z, self.min.z)
        };
        let tmin_x = (min_x - origin.x) * inv_dir.x;
        let tmin_y = (min_y - origin.y) * inv_dir.y;
        let tmin_z = (min_z - origin.z) * inv_dir.z;
        let tmax_x = (max_x - origin.x) * inv_dir.x;
        let tmax_y = (max_y - origin.y) * inv_dir.y;
        let tmax_z = (max_z - origin.z) * inv_dir.z;

        (
            tmin_x.max(tmin_y).max(tmin_z),
            tmax_z.min(tmax_y).min(tmax_x),
        )
    }

    #[inline(always)]
    fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }
}

// TODO: Test intersects_ray_at
// TODO: Test intersects
