#![feature(portable_simd)]

pub extern crate ultraviolet as uv;

mod aa;
pub mod algo;
pub mod camera;
mod pixels;
pub mod ray;
pub mod sphere;

pub use pixels::*;

/// Signed Distance Field.
///
/// Used to compute the signed distance from a point in space.
pub trait Sdf {
    /// The signed distance from point `p`.
    fn dist(&self, p: &uv::Vec3x16) -> uv::f32x16;
}

/// The volm prelude.
///
/// Imports useful types, functions and traits.
pub mod prelude {
    pub use super::camera::{Camera, PointCamera};
    pub use crate::Sdf;
    pub use crate::Splat;
}

/// Allows a type to be "splatted" over multiple lanes of a vector.
pub trait Splat {
    type Output;

    fn splat(self) -> Self::Output;
}

impl Splat for f32 {
    type Output = uv::f32x16;

    fn splat(self) -> Self::Output {
        uv::f32x16::splat(self)
    }
}

impl Splat for u32 {
    type Output = uv::simd::u32x16;

    fn splat(self) -> Self::Output {
        uv::simd::u32x16::splat(self)
    }
}

impl Splat for i32 {
    type Output = uv::simd::i32x16;

    fn splat(self) -> Self::Output {
        uv::simd::i32x16::splat(self)
    }
}
