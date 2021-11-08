#![feature(portable_simd)]

pub mod math;

/// The math prelude.
/// 
/// Imports useful types, functions and traits.
pub mod prelude {
    pub use crate::math::*;
}