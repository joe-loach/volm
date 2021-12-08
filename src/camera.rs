use uv::{Vec2x8, Vec3x8};

use crate::ray::Ray;

pub trait Camera: Send + Sync {
    /// Computes the Ray sent out at the UV coordinate.
    ///
    /// UV coords are the same as Vulkan texture coords.
    fn ray_at(&self, uv: Vec2x8) -> Ray;
}

/// A [`PointCamera`].
///
/// The camera has an origin of `pos` and always points along the negative Z axis.
pub struct PointCamera {
    pos: Vec3x8,
}

impl PointCamera {
    #[inline]
    pub fn new(pos: Vec3x8) -> Self {
        Self { pos }
    }
}

impl Camera for PointCamera {
    #[inline]
    fn ray_at(&self, uv: Vec2x8) -> Ray {
        Ray::new(
            self.pos,
            Vec3x8::new(uv.x, uv.y, uv::f32x8::splat(-2.0)).normalized(),
        )
    }
}
