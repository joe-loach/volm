#![feature(portable_simd)]

pub extern crate ultraviolet as uv;

pub mod camera;
pub mod ray;

/// Signed Distance Field.
/// 
/// Used to compute the signed distance from a point in space.
pub trait Sdf {
    /// The signed distance from point `p`.
    fn dist(&self, p: &uv::Vec3x8) -> uv::f32x8;
}

/// The volm prelude.
/// 
/// Imports useful types, functions and traits.
pub mod prelude {
    pub use crate::Sdf;
}