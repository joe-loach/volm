/// An infinitely long ray pointing in a `direction` from a given `origin`.
pub struct Ray {
    origin: uv::Vec3x16,
    dir: uv::Vec3x16,
}

impl Ray {
    /// Creates a new [`Ray`].
    #[inline]
    pub fn new(origin: uv::Vec3x16, dir: uv::Vec3x16) -> Self {
        Self { origin, dir }
    }

    /// Gets a point distance `t` along the [`Ray`].
    #[inline]
    pub fn at(&self, t: uv::f32x16) -> uv::Vec3x16 {
        // self.origin + self.dir * t
        self.dir.mul_add(uv::Vec3x16::broadcast(t), self.origin)
    }
}
