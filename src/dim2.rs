//! A module with implementations for 2D support

use crate::morton::{morton_encode, MORTON_CENTER};
use crate::{Aabb, Bvh, Dim, Pos};

pub use bevy_math::Vec2;

/// A 2D BVH
pub type Bvh2d<T> = Bvh<Dim2, T>;

/// An implementation of 2D
#[derive(Clone, Copy, Debug)]
pub struct Dim2;

impl Dim for Dim2 {
    type Aabb = Aabb2d;
}

impl Pos for Vec2 {
    const ZERO: Self = Self::ZERO;

    #[inline(always)]
    fn code(self) -> usize {
        morton_encode(
            (self.x + MORTON_CENTER) as usize,
            (self.y + MORTON_CENTER) as usize,
            0,
            5,
        )
    }

    #[inline(always)]
    fn inverse(self) -> Self {
        1. / self
    }
}

/// A 2d Axis-Aligned Bounding Box. Since it's 2d, it's really just a rectangle
#[derive(Clone, Copy, Debug)]
pub struct Aabb2d {
    min: Vec2,
    max: Vec2,
}

impl From<(Vec2, Vec2)> for Aabb2d {
    #[inline(always)]
    fn from((min, max): (Vec2, Vec2)) -> Self {
        Self { min, max }
    }
}

impl Aabb for Aabb2d {
    type Pos = Vec2;

    const INFINITY: Self = Self {
        min: Vec2::splat(-f32::INFINITY),
        max: Vec2::splat(f32::INFINITY),
    };

    #[inline(always)]
    fn center(&self) -> Self::Pos {
        self.max - self.min
    }

    #[inline(always)]
    fn area(&self) -> f32 {
        let b = self.max - self.min;
        b.x * b.y
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
        let tmin_x = (min_x - origin.x) * inv_dir.x;
        let tmin_y = (min_y - origin.y) * inv_dir.y;
        let tmax_x = (max_x - origin.x) * inv_dir.x;
        let tmax_y = (max_y - origin.y) * inv_dir.y;

        (tmin_x.max(tmin_y), tmax_y.min(tmax_x))
    }

    #[inline(always)]
    fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

// TODO: Test intersects_ray_at
// TODO: Test intersects
