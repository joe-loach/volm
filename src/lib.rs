#![feature(portable_simd)]

pub mod math;

use math::*;

/// Signed Distance Field.
/// 
/// Used to compute the signed distance from a point in space.
pub trait Sdf {
    /// The signed distance from point `p`.
    fn dist(&self, p: &Vec3s) -> f32s;
}

/// The math prelude.
/// 
/// Imports useful types, functions and traits.
pub mod prelude {
    pub use crate::math::*;
}