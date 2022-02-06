use crate::{Sdf, Splat};

/// A sphere with a given `radius` centered on the origin.
pub struct Sphere {
    radius: f32,
}

impl Sphere {
    #[inline]
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Sdf for Sphere {
    #[inline]
    fn dist(&self, p: &uv::Vec3x16) -> uv::f32x16 {
        p.mag() - self.radius.splat()
    }
}
