#![allow(non_camel_case_types)]

mod vec2;
mod vec3;
mod splat;
mod fma;
mod ray;

pub use splat::Splattable;
pub(crate) use fma::Fma;

const LANES: usize = 16;

pub type f32s = core_simd::Simd<f32, LANES>;
pub type u32s = core_simd::Simd<u32, LANES>;
pub type mask32s = core_simd::Mask<i32, LANES>;

/// A two dimensional vector with [`f32s`] components.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2s {
    /// The `x` component of the vector.
    pub x: f32s,
    /// The `y` component of the vector.
    pub y: f32s,
}

/// A three dimensional vector with [`f32s`] components.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3s {
    /// The `x` component of the vector.
    pub x: f32s,
    /// The `y` component of the vector.
    pub y: f32s,
    /// The `z` component of the vector.
    pub z: f32s,
}

pub struct Ray {
    /// The origin of the ray.
    pub origin: Vec3s,
    /// The direction the ray is travelling.
    pub dir: Vec3s,
}